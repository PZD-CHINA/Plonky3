use std::array;

use p3_baby_bear::{BabyBear, MdsMatrixBabyBear};
use p3_field::{AbstractField, Field, PrimeField};
use p3_goldilocks::{Goldilocks, MdsMatrixGoldilocks};
use p3_mds::coset_mds::CosetMds;
use p3_mds::MdsPermutation;
use p3_mersenne_31::{MdsMatrixMersenne31, Mersenne31};
use p3_poseidon::Poseidon;
use p3_symmetric::Permutation;
use rand::distributions::{Distribution, Standard};
use rand::thread_rng;

fn main() {
    test_poseidon::<BabyBear, MdsMatrixBabyBear, 16, 7>();
    test_poseidon::<BabyBear, MdsMatrixBabyBear, 24, 7>();
    test_poseidon::<BabyBear, CosetMds<_, 32>, 32, 7>();
    test_poseidon::<<BabyBear as Field>::Packing, CosetMds<_, 32>, 32, 7>();

    test_poseidon::<Goldilocks, MdsMatrixGoldilocks, 8, 7>();
    test_poseidon::<Goldilocks, MdsMatrixGoldilocks, 12, 7>();
    test_poseidon::<Goldilocks, MdsMatrixGoldilocks, 16, 7>();

    test_poseidon::<Mersenne31, MdsMatrixMersenne31, 16, 5>();
    test_poseidon::<Mersenne31, MdsMatrixMersenne31, 32, 5>();
}

fn test_poseidon<AF, Mds, const WIDTH: usize, const ALPHA: u64>()
where
    AF: AbstractField,
    AF::F: PrimeField,
    Standard: Distribution<AF::F>,
    Mds: MdsPermutation<AF, WIDTH> + Default,
{
    let mut rng = thread_rng();
    let mds = Mds::default();
    let half_num_full_rounds = 4;
    let num_partial_rounds = 22;

    let poseidon = Poseidon::<AF::F, Mds, WIDTH, ALPHA>::new_from_rng(
        half_num_full_rounds,
        num_partial_rounds,
        mds,
        &mut rng,
    );
    let input: [AF; WIDTH] = array::from_fn(|_| AF::zero());
    let result = poseidon.permute(input.clone());

    println!("Poseidon Hash for {}x{}:", WIDTH, ALPHA);
    println!("Input: {:?}", input);
    println!("Output: {:?}", result);
}

