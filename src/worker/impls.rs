use crate::solver::impls::Solver;
use tokio::task;

pub async fn async_solve(solver: Solver, x: i32, y: i32) {
    let result = task::spawn_blocking(move || solver.solve(x, y))
        .await
        .unwrap();
    println!("Async result: {}", result);
}
