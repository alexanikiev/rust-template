use rusttemplate::solver::impls::Solver;

fn main() {
    let solver = Solver::new("example_a", "example_b");

    let x = 5;
    let y = 10;

    println!("Solving with x={} and y={}", x, y);
    let result = solver.solve(x, y);
    println!("Result: {}", result);

    match solver.error() {
        Ok(_) => println!("No error"),
        Err(e) => println!("Error: {}", e),
    }
}
