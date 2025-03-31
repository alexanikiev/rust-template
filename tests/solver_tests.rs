use rusttemplate::solver::impls::Solver;

#[test]
fn test_solve() {
    let solver = Solver::new("a", "b");
    assert_eq!(solver.solve(3, 4), 12);
}

#[test]
fn test_solve_with_zero() {
    let solver = Solver::new("a", "b");
    assert_eq!(solver.solve(0, 5), 0);
}

#[test]
fn test_solve_with_negative_numbers() {
    let solver = Solver::new("a", "b");
    assert_eq!(solver.solve(-3, 4), -12);
    assert_eq!(solver.solve(-3, -4), 12);
}

// #[test]
// fn test_error() {
//     let solver = Solver::new("a", "b");
//     assert!(solver.error().is_err());
// }
