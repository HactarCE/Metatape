#[macro_use]
extern crate pest_derive;

mod metatape;

fn main() -> Result<(), crate::metatape::parser::ParseError> {
    // while let Ok(()) = runtime.step() {}
    Ok(())
}
