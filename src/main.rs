use std::error::Error;

use report_checker::grammar_check;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Starting.");
    grammar_check(&"Hi!");
    println!("Finished.");
    Ok(())
}
