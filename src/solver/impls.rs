pub struct Solver {
    pub a: String,
    pub b: String,
}

impl Solver {
    /// Creates a new Solver instance
    pub fn new(a: &str, b: &str) -> Self {
        Self {
            a: a.to_string(),
            b: b.to_string(),
        }
    }

    /// Private method to perform the calculation
    fn calculate(&self, x: i32, y: i32) -> i32 {
        x * y
    }

    /// Public method to solve the problem
    pub fn solve(&self, x: i32, y: i32) -> i32 {
        self.calculate(x, y)
    }

    /// Method that raises an error based on input
    pub fn error(&self) -> Result<(), String> {
        if self.a == "fail" {
            Err("This is an error message.".to_string())
        } else {
            Ok(())
        }
    }
}
