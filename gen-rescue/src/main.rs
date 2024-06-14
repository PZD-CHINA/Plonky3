use std::any::type_name;
use std::array;

use p3_baby_bear::{BabyBear, MdsMatrixBabyBear};
use p3_field::{AbstractField, Field, PrimeField64};
use p3_goldilocks::{Goldilocks, MdsMatrixGoldilocks};
use p3_mds::integrated_coset_mds::IntegratedCosetMds;
use p3_mds::MdsPermutation;
use p3_mersenne_31::{MdsMatrixMersenne31, Mersenne31};
use p3_rescue::{BasicSboxLayer, Rescue};
use p3_symmetric::Permutation;
use rand::distributions::{Distribution, Standard};
use rand::{thread_rng, Rng};

fn main() {
    test_rescue::<BabyBear, IntegratedCosetMds<_, 16>, 16, 7>();
    test_rescue::<<BabyBear as Field>::Packing, IntegratedCosetMds<_, 16>, 16, 7>();
    test_rescue::<BabyBear, MdsMatrixBabyBear, 24, 7>();
    test_rescue::<BabyBear, MdsMatrixBabyBear, 32, 7>();

    test_rescue::<Goldilocks, MdsMatrixGoldilocks, 8, 7>();
    test_rescue::<Goldilocks, MdsMatrixGoldilocks, 12, 7>();
    test_rescue::<Goldilocks, MdsMatrixGoldilocks, 16, 7>();

    test_rescue::<Mersenne31, MdsMatrixMersenne31, 16, 5>();
    test_rescue::<Mersenne31, MdsMatrixMersenne31, 32, 5>();
}

fn test_rescue<AF, Mds, const WIDTH: usize, const ALPHA: u64>()
where
    AF: AbstractField,
    AF::F: PrimeField64,
    Standard: Distribution<AF::F>,
    Mds: MdsPermutation<AF, WIDTH> + Default,
{
    const NUM_ROUNDS: usize = 8;
    let mut rng = thread_rng();
    let num_constants = 2 * WIDTH * NUM_ROUNDS;
    let round_constants: Vec<AF::F> = rng.sample_iter(Standard).take(num_constants).collect();
    let mds = Mds::default();
    let sbox = BasicSboxLayer::for_alpha(ALPHA);
    let rescue = Rescue::<AF::F, Mds, _, WIDTH>::new(NUM_ROUNDS, round_constants, mds, sbox);
    let input: [AF; WIDTH] = array::from_fn(|_| AF::zero());
    let result = rescue.permute(input.clone());

    println!("Rescue Hash for {}x{}:", WIDTH, ALPHA);
    println!("Input: {:?}", input);
    println!("Output: {:?}", result);
}
