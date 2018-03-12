extern crate klotski;

use std::time::Instant;

use klotski::generic_solver::*;
use klotski::klotski::*;

fn main() {
    let initial = Klotski::initial();
    let solver = Solver::<Klotski>::new(initial);

    println!("Solving {:?}...", initial);

    let before = Instant::now();
    let result = solver.solve().expect("a solution");;
    let duration = before.elapsed();

    println!("Solution: {:?}", result);
    println!("Solved in {} seconds", duration.as_secs());
}