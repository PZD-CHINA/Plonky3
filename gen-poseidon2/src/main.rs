use std::any::type_name;

use p3_baby_bear::{BabyBear, DiffusionMatrixBabyBear};
use p3_bn254_fr::{Bn254Fr, DiffusionMatrixBN254};
use p3_field::PrimeField;
use p3_goldilocks::{DiffusionMatrixGoldilocks, Goldilocks};
use p3_koala_bear::{DiffusionMatrixKoalaBear, KoalaBear};
use p3_mersenne_31::{DiffusionMatrixMersenne31, Mersenne31};
use p3_poseidon2::{
    DiffusionPermutation, MdsLightPermutation, Poseidon2, Poseidon2ExternalMatrixGeneral,
};
use p3_symmetric::Permutation;
use rand::distributions::{Distribution, Standard};
use rand::thread_rng;

fn main() {
    test_poseidon2::<BabyBear, Poseidon2ExternalMatrixGeneral, DiffusionMatrixBabyBear, 16, 7>(8, 22);
    test_poseidon2::<BabyBear, Poseidon2ExternalMatrixGeneral, DiffusionMatrixBabyBear, 24, 7>(8, 22);

    test_poseidon2::<KoalaBear, Poseidon2ExternalMatrixGeneral, DiffusionMatrixKoalaBear, 16, 3>(8, 22);
    test_poseidon2::<KoalaBear, Poseidon2ExternalMatrixGeneral, DiffusionMatrixKoalaBear, 16, 5>(8, 22);
    test_poseidon2::<KoalaBear, Poseidon2ExternalMatrixGeneral, DiffusionMatrixKoalaBear, 16, 7>(8, 22);
    test_poseidon2::<KoalaBear, Poseidon2ExternalMatrixGeneral, DiffusionMatrixKoalaBear, 24, 3>(8, 22);
    test_poseidon2::<KoalaBear, Poseidon2ExternalMatrixGeneral, DiffusionMatrixKoalaBear, 24, 5>(8, 22);
    test_poseidon2::<KoalaBear, Poseidon2ExternalMatrixGeneral, DiffusionMatrixKoalaBear, 24, 7>(8, 22);

    test_poseidon2::<Mersenne31, Poseidon2ExternalMatrixGeneral, DiffusionMatrixMersenne31, 16, 5>(8, 22);
    test_poseidon2::<Mersenne31, Poseidon2ExternalMatrixGeneral, DiffusionMatrixMersenne31, 24, 5>(8, 22);

    test_poseidon2::<Goldilocks, Poseidon2ExternalMatrixGeneral, DiffusionMatrixGoldilocks, 8, 7>(8, 22);
    test_poseidon2::<Goldilocks, Poseidon2ExternalMatrixGeneral, DiffusionMatrixGoldilocks, 12, 7>(8, 22);
    test_poseidon2::<Goldilocks, Poseidon2ExternalMatrixGeneral, DiffusionMatrixGoldilocks, 16, 7>(8, 22);

    test_poseidon2::<Bn254Fr, Poseidon2ExternalMatrixGeneral, DiffusionMatrixBN254, 3, 5>(8, 22);

}

fn test_poseidon2<F, MdsLight, Diffusion, const WIDTH: usize, const D: u64>(
    rounds_f: usize,
    rounds_p: usize,
) where
    F: PrimeField,
    Standard: Distribution<F>,
    MdsLight: MdsLightPermutation<F, WIDTH> + Default,
    Diffusion: DiffusionPermutation<F, WIDTH> + Default,
{
    let mut rng = thread_rng();
    let external_linear_layer = MdsLight::default();
    let internal_linear_layer = Diffusion::default();

    let poseidon = Poseidon2::<F, MdsLight, Diffusion, WIDTH, D>::new_from_rng(
        rounds_f,
        external_linear_layer,
        rounds_p,
        internal_linear_layer,
        &mut rng,
    );
    let input: [F; WIDTH] = [F::zero(); WIDTH];
    let result: [F; WIDTH] = poseidon.permute(input.clone());

    println!("Poseidon2::<{}, {}, {}, {}>:", type_name::<F>(), D, rounds_f, rounds_p);
    println!("Input: {:?}", input);
    println!("Result: {:?}", result);
}

// You should update test_poseidon2 in a similar manner if you use that function.
