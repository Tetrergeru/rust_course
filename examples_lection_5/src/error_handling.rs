use std::{error::Error, fmt::Display};

pub fn solve_quadratic_1(a: f64, b: f64, c: f64) -> (f64, f64) {
    let d = b * b - 4.0 * a * c;
    let d_dqrt = d.sqrt();

    let x_1 = (-b + d_dqrt) / (2.0 * a);
    let x_2 = (-b - d_dqrt) / (2.0 * a);

    (x_1, x_2)
}

pub const EPS: f64 = 0.00001;

#[test]
fn test_solve_quadratic_1() {
    let (x_1, x_2) = solve_quadratic_1(1.0, 1.0, -2.0);

    assert!((x_1 - 1.0).abs() < EPS);
    assert!((x_2 + 2.0).abs() < EPS);

    let (x_1, x_2) = solve_quadratic_1(1.0, 2.0, 1.0);

    assert!((x_1 + 1.0).abs() < EPS);
    assert!((x_2 + 1.0).abs() < EPS);

    let (x_1, x_2) = solve_quadratic_1(0.0, 2.0, 1.0);
    // ???
    println!("x_1 = {x_1}, x_2 = {x_2}");

    let (x_1, x_2) = solve_quadratic_1(1.0, 3.0, 3.0);
    // ???
    println!("x_1 = {x_1}, x_2 = {x_2}");
}

pub fn solve_quadratic_2(a: f64, b: f64, c: f64) -> Option<(f64, f64)> {
    if a == 0.0 {
        panic!("Expected a quadratic equation");
    }

    let d = b * b - 4.0 * a * c;

    if d < 0.0 {
        return None;
    }

    let d_dqrt = d.sqrt();

    let x_1 = (-b + d_dqrt) / (2.0 * a);
    let x_2 = (-b - d_dqrt) / (2.0 * a);

    Some((x_1, x_2))
}

#[test]
fn test_solve_quadratic_2() {
    let Some((x_1, x_2)) = solve_quadratic_2(1.0, 1.0, -2.0) else {
        panic!("Expected a correct solution");
    };

    assert!((x_1 - 1.0).abs() < EPS);
    assert!((x_2 + 2.0).abs() < EPS);

    let Some((x_1, x_2)) = solve_quadratic_2(1.0, 2.0, 1.0) else {
        panic!("Expected a correct solution");
    };

    assert!((x_1 + 1.0).abs() < EPS);
    assert!((x_2 + 1.0).abs() < EPS);

    let v = solve_quadratic_2(1.0, 3.0, 3.0);
    assert_eq!(v, None);
}

#[test]
#[should_panic]
fn test_solve_quadratic_2_panic() {
    solve_quadratic_2(0.0, 2.0, 1.0);
}

#[derive(PartialEq, Debug)]
pub enum QuadraticSolverError {
    EquationWasLinear,
}

type Result<T> = core::result::Result<T, QuadraticSolverError>;

pub fn solve_quadratic_3(a: f64, b: f64, c: f64) -> Result<Option<(f64, f64)>> {
    if a == 0.0 {
        return Err(QuadraticSolverError::EquationWasLinear);
    }

    let d = b * b - 4.0 * a * c;

    if d < 0.0 {
        return Ok(None);
    }

    let d_dqrt = d.sqrt();

    let x_1 = (-b + d_dqrt) / (2.0 * a);
    let x_2 = (-b - d_dqrt) / (2.0 * a);

    Ok(Some((x_1, x_2)))
}

#[test]
fn test_solve_quadratic_3() {
    let Ok(Some((x_1, x_2))) = solve_quadratic_3(1.0, 1.0, -2.0) else {
        panic!("Expected a correct solution");
    };

    assert!((x_1 - 1.0).abs() < EPS);
    assert!((x_2 + 2.0).abs() < EPS);

    let Ok(Some((x_1, x_2))) = solve_quadratic_3(1.0, 2.0, 1.0) else {
        panic!("Expected a correct solution");
    };

    assert!((x_1 + 1.0).abs() < EPS);
    assert!((x_2 + 1.0).abs() < EPS);

    let v = solve_quadratic_3(1.0, 3.0, 3.0);
    assert_eq!(v, Ok(None));

    let v = solve_quadratic_3(0.0, 2.0, 1.0);
    assert_eq!(v, Err(QuadraticSolverError::EquationWasLinear));
}


#[derive(PartialEq, Debug)]
pub enum QuadraticSolverError2 {
    EquationWasLinear,
    NoRealSolutions,
}

type Result2<T> = core::result::Result<T, QuadraticSolverError2>;

pub fn solve_quadratic_4(a: f64, b: f64, c: f64) -> Result2<(f64, f64)> {
    if a == 0.0 {
        return Err(QuadraticSolverError2::EquationWasLinear);
    }

    let d = b * b - 4.0 * a * c;

    if d < 0.0 {
        return Err(QuadraticSolverError2::NoRealSolutions);
    }

    let d_dqrt = d.sqrt();

    let x_1 = (-b + d_dqrt) / (2.0 * a);
    let x_2 = (-b - d_dqrt) / (2.0 * a);

    Ok((x_1, x_2))
}
