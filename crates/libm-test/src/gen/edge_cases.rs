use crate::{Float, GenerateInput, Int};

fn float_edge_cases<F: Float>() -> impl Iterator<Item = F> + Clone {
    let exp_cases = [
        F::Int::ZERO,
        F::Int::ONE,
        F::Int::ONE << (F::EXPONENT_BITS / 2),
        (F::Int::ONE << (F::EXPONENT_BITS - 1)) - F::Int::ONE,
        F::Int::ONE << (F::EXPONENT_BITS - 1),
        (F::Int::ONE << (F::EXPONENT_BITS - 1)) + F::Int::ONE,
        // Saturated exponent for infinity and NaN
        (F::Int::ONE << F::EXPONENT_BITS) - F::Int::ONE,
        // Exponent for min and max values of the float
        (F::Int::ONE << F::EXPONENT_BITS) - F::Int::ONE - F::Int::ONE,
    ];
    let mant_cases = [
        F::Int::ZERO,
        F::Int::ONE,
        F::Int::ONE << (F::SIGNIFICAND_BITS / 2),
        (F::Int::ONE << (F::SIGNIFICAND_BITS - 1)) - F::Int::ONE,
        F::Int::ONE << (F::SIGNIFICAND_BITS - 1),
        (F::Int::ONE << (F::SIGNIFICAND_BITS - 1)) + F::Int::ONE,
        // Saturated mantissa
        (F::Int::ONE << F::SIGNIFICAND_BITS) - F::Int::ONE,
    ];
    let sign_cases = [false, true];

    exp_cases
        .into_iter()
        .flat_map(move |exp| mant_cases.into_iter().map(move |sig| (exp, sig)))
        .flat_map(move |(exp, mant)| sign_cases.into_iter().map(move |sign| (exp, mant, sign)))
        .map(|(exp, mant, sign)| F::from_parts(sign, exp, mant))
}

pub struct EdgeCases;

impl<F: Float> GenerateInput<(F,)> for EdgeCases {
    fn get_cases(&self) -> impl Iterator<Item = (F,)> {
        float_edge_cases::<F>().map(|f| (f,))
    }
}

impl<F: Float> GenerateInput<(F, F)> for EdgeCases {
    fn get_cases(&self) -> impl Iterator<Item = (F, F)> {
        float_edge_cases::<F>().flat_map(|f1| float_edge_cases::<F>().map(move |f2| (f1, f2)))
    }
}

impl<F: Float> GenerateInput<(F, F, F)> for EdgeCases {
    fn get_cases(&self) -> impl Iterator<Item = (F, F, F)> {
        #[cfg(feature = "test-exhaustive")]
        let iter = float_edge_cases::<F>()
            .flat_map(|f1| float_edge_cases::<F>().map(move |f2| (f1, f2)))
            .flat_map(|(f1, f2)| float_edge_cases::<F>().map(move |f3| (f1, f2, f3)));

        // Three inputs blows up fast, so limit to 16*16*16 (4096)
        #[cfg(not(feature = "test-exhaustive"))]
        let iter = {
            let cases1 = float_edge_cases::<F>().take(16);
            let cases2 = cases1.clone();
            let cases3 = cases2.clone();

            cases1
                .flat_map(move |f1| cases2.clone().map(move |f2| (f1, f2)))
                .flat_map(move |(f1, f2)| cases3.clone().map(move |f3| (f1, f2, f3)))
        };

        iter
    }
}

impl<F: Float> GenerateInput<(F, i32)> for EdgeCases {
    fn get_cases(&self) -> impl Iterator<Item = (F, i32)> {
        // todo!()
        [].into_iter()
        // float_edge_cases::<F>().map(|f| (f,))
    }
}

impl<F: Float> GenerateInput<(i32, F)> for EdgeCases {
    fn get_cases(&self) -> impl Iterator<Item = (i32, F)> {
        // todo!()
        [].into_iter()
        // float_edge_cases::<F>().map(|f| (f,))
    }
}

pub fn get_test_cases<RustArgs>(_fname: &str) -> impl Iterator<Item = RustArgs>
where
    EdgeCases: GenerateInput<RustArgs>,
{
    // let inputs = if fname == "jn" || fname == "jnf" {
    //     &TEST_CASES_JN
    // } else {
    //     &TEST_CASES
    // };

    EdgeCases.get_cases()
}
