use rusttemplate::solver::impls::Solver;

fn main() {
    let solver = Solver::new("a", "b");

    let x = 3;
    let y = 4;

    println!("Solving with x={} and y={}", x, y);
    let result = solver.solve(x, y);
    println!("Result: {}", result);

    match solver.error() {
        Ok(_) => println!("No error"),
        Err(e) => println!("Error: {}", e),
    }
}
