use std::{error::Error, fmt::Display};

use crate::error_handling::{solve_quadratic_4, QuadraticSolverError2, EPS};

#[derive(PartialEq, Debug)]
pub struct EvenNumber(i32);

#[derive(PartialEq, Debug)]
pub enum EvenNumberError {
    NumberWasNotInteger,
    NumberWasNotEven,
}

impl EvenNumber {
    pub fn new(number: f64) -> Result<Self, EvenNumberError> {
        let int_number = number as i32;
        if (number - int_number as f64).abs() > EPS {
            Err(EvenNumberError::NumberWasNotInteger)
        } else if int_number % 2 != 0 {
            Err(EvenNumberError::NumberWasNotEven)
        } else {
            Ok(Self(int_number))
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum MathError {
    QuadraticSolverError(QuadraticSolverError2),
    NumberWasNotInteger,
    NumberWasNotEven,
    ExpectedOneRootGotTwo,
}

pub fn solve_quadratic_one_even_root_1(a: f64, b: f64, c: f64) -> Result<EvenNumber, MathError> {
    let v = solve_quadratic_4(a, b, c);

    let (x_1, x_2) = match v {
        Ok((x_1, x_2)) => (x_1, x_2),
        Err(err) => return Err(MathError::QuadraticSolverError(err)),
    };

    if (x_1 - x_2).abs() > EPS {
        return Err(MathError::ExpectedOneRootGotTwo);
    }

    let even = EvenNumber::new(x_1);

    match even {
        Ok(even) => Ok(even),
        Err(EvenNumberError::NumberWasNotEven) => Err(MathError::NumberWasNotEven),
        Err(EvenNumberError::NumberWasNotInteger) => Err(MathError::NumberWasNotInteger),
    }
}

#[test]
fn test_solve_quadratic_one_even_root_1() {
    let v = solve_quadratic_one_even_root_1(1.0, 3.0, 3.0);
    assert_eq!(
        v,
        Err(MathError::QuadraticSolverError(
            QuadraticSolverError2::NoRealSolutions
        ))
    );

    let v = solve_quadratic_one_even_root_1(0.0, 3.0, 3.0);
    assert_eq!(
        v,
        Err(MathError::QuadraticSolverError(
            QuadraticSolverError2::EquationWasLinear
        ))
    );

    let v = solve_quadratic_one_even_root_1(1.0, 1.0, -2.0);
    assert_eq!(v, Err(MathError::ExpectedOneRootGotTwo));

    let v = solve_quadratic_one_even_root_1(1.0, 1.0, 0.25);
    assert_eq!(v, Err(MathError::NumberWasNotInteger));

    let v = solve_quadratic_one_even_root_1(1.0, 2.0, 1.0);
    assert_eq!(v, Err(MathError::NumberWasNotEven));

    let v = solve_quadratic_one_even_root_1(1.0, 4.0, 4.0);
    assert_eq!(v, Ok(EvenNumber(-2)));
}

impl From<EvenNumberError> for MathError {
    fn from(value: EvenNumberError) -> Self {
        use EvenNumberError::*;

        match value {
            NumberWasNotInteger => MathError::NumberWasNotInteger,
            NumberWasNotEven => MathError::NumberWasNotEven,
        }
    }
}

impl From<QuadraticSolverError2> for MathError {
    fn from(value: QuadraticSolverError2) -> Self {
        MathError::QuadraticSolverError(value)
    }
}

pub fn solve_quadratic_one_even_root_2(a: f64, b: f64, c: f64) -> Result<EvenNumber, MathError> {
    let (x_1, x_2) = solve_quadratic_4(a, b, c)?;

    if (x_1 - x_2).abs() > EPS {
        return Err(MathError::ExpectedOneRootGotTwo);
    }

    Ok(EvenNumber::new(x_1)?)
}

impl Display for EvenNumberError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{self:?}"))
    }
}

impl Error for EvenNumberError {}

impl Display for QuadraticSolverError2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{self:?}"))
    }
}

impl Error for QuadraticSolverError2 {}

impl Display for MathError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{self:?}"))
    }
}

impl Error for MathError {}

pub fn solve_quadratic_one_even_root_3(a: f64, b: f64, c: f64) -> anyhow::Result<EvenNumber> {
    let (x_1, x_2) = solve_quadratic_4(a, b, c)?;

    if (x_1 - x_2).abs() > EPS {
        return Err(MathError::ExpectedOneRootGotTwo)?;
    }

    Ok(EvenNumber::new(x_1)?)
}
