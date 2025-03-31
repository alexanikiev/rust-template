use rusttemplate::solver::impls::Solver;
use rusttemplate::worker::impls::async_solve;
use tokio::runtime::Runtime;

fn main() {
    let solver = Solver::new("a", "b");
    let rt = Runtime::new().unwrap();
    rt.block_on(async_solve(solver, 3, 4));
}
