use alloc::vec::Vec;
use core::borrow::Borrow;
use core::mem::size_of;

use p3_air::{Air, AirBuilder, BaseAir};
use p3_field::AbstractField;
use p3_matrix::MatrixRowSlices;
use p3_mds::MdsPermutation;

use crate::columns::PoseidonCols;
use crate::get_num_poseidon_cols;
use crate::round_flags::eval_round_flags;

pub struct PoseidonAir<Mds: Sync, const WIDTH: usize, const ALPHA: u64, const N_ROUNDS: usize> {
    half_num_full_rounds: usize,
    num_partial_rounds: usize,
    round_constants: Vec<u64>,
    mds: Mds,
}

impl<Mds: Sync, const WIDTH: usize, const ALPHA: u64, const N_ROUNDS: usize>
    PoseidonAir<Mds, WIDTH, ALPHA, N_ROUNDS>
{
    pub fn new(
        half_num_full_rounds: usize,
        num_partial_rounds: usize,
        round_constants: Vec<u64>,
        mds: Mds,
    ) -> Self {
        let num_rounds = 2 * half_num_full_rounds + num_partial_rounds;
        assert_eq!(num_rounds, N_ROUNDS);
        assert_eq!(round_constants.len(), WIDTH * num_rounds);

        Self {
            half_num_full_rounds,
            num_partial_rounds,
            round_constants,
            mds,
        }
    }
}

impl<F, Mds: Sync, const WIDTH: usize, const ALPHA: u64, const N_ROUNDS: usize> BaseAir<F>
    for PoseidonAir<Mds, WIDTH, ALPHA, N_ROUNDS>
{
    fn width(&self) -> usize {
        get_num_poseidon_cols!(WIDTH, N_ROUNDS)
    }
}

impl<AB: AirBuilder, Mds: Sync, const WIDTH: usize, const ALPHA: u64, const N_ROUNDS: usize> Air<AB>
    for PoseidonAir<Mds, WIDTH, ALPHA, N_ROUNDS>
where
    Mds: MdsPermutation<AB::Expr, WIDTH>,
{
    fn eval(&self, builder: &mut AB) {
        let num_rounds = 2 * self.half_num_full_rounds + self.num_partial_rounds;
        assert_eq!(num_rounds, N_ROUNDS);

        eval_round_flags::<AB, WIDTH, N_ROUNDS>(builder);

        let main = builder.main();
        let local: &PoseidonCols<AB::Var, WIDTH, N_ROUNDS> = main.row_slice(0).borrow();
        let next: &PoseidonCols<AB::Var, WIDTH, N_ROUNDS> = main.row_slice(1).borrow();

        // The partial round flag must be 0 or 1.
        builder.assert_bool(local.partial_round);

        // check that round constants are added correctly
        let constants = self.round_constants.clone();
        for i in 0..WIDTH {
            let mut round_constant = AB::Expr::zero();
            for r in 0..num_rounds {
                let this_round = local.round_flags[r];
                let this_round_constant = AB::Expr::from_canonical_u64(constants[r * WIDTH + i]);
                round_constant += this_round * this_round_constant;
            }
            let before = local.start_of_round[i];
            let expected = local.after_constants[i];

            builder.assert_eq(expected, before + round_constant);
        }

        // check that sbox layer is correct
        // partial s-box
        let before = local.after_constants[0];
        let expected = local.after_sbox[0];
        let after = before.into().exp_u64(ALPHA);
        builder.assert_eq(expected, after);

        // full s-box
        let full_round = AB::Expr::one() - local.partial_round;
        for i in 0..WIDTH {
            let before = local.after_constants[i];
            let expected = local.after_sbox[i];
            let after = before.into().exp_u64(ALPHA);
            builder.assert_eq(after * full_round.clone(), expected * full_round.clone());
        }

        // check that MDS layer is correct
        let before: [AB::Expr; WIDTH] = local.after_sbox.map(|x| x.into());
        let expected = local.after_mds;
        let after = self.mds.permute(before);
        for i in 0..WIDTH {
            builder.assert_eq(after[i].clone(), expected[i]);
        }

        // check that end of this round matches start of next round
        for i in 0..WIDTH {
            let end = local.after_mds[i];
            let start = next.start_of_round[i];
            builder.assert_eq(end, start);
        }
    }
}
