use std::error::Error;

use harper_core::Dialect;
use report_checker::grammar_check;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Starting.");

    match std::fs::read_to_string("./samples/test.txt")
    {
        Ok(input)=>{
            grammar_check(&input, Dialect::American, None, None);
        },
        Err(e)=>{
            return Err(Box::new(e));
        }
    }
    println!("Finished.");
    Ok(())
}
