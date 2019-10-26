#[macro_use]
extern crate pest_derive;

mod metatape;

fn main() -> Result<(), crate::metatape::parser::ParseError> {
    //     println!("Hello, world!");

    //     println!(
    //         "{:#?}",
    //         metatape::parser::parse(
    //             "
    // ex<
    // !h !e !l !l !o !_ !w !o !r !l !d !!
    // @ _ { oo>o<ooooo }
    // @ ! { oo>o<oooo>o< }
    // @ d { o>oo<oo>o<oo }
    // @ e { o>oo<oo>o<o>o< }
    // @ h { o>o<oo>o<ooo }
    // @ l { o>oo<o>oo<oo }
    // @ o { o>oo<o>oooo< }
    // @ r { o>ooo<oo>o<o }
    // @ w { o>ooo<o>ooo< }
    // "
    //         )
    //     );

    let program = metatape::parser::parse(
        "
[
ex>e
ex<<<<<<<<
[eexix>(|])
[<(])
[>eo(xx<n>e|x)]
xn<(])
",
    )?;

    let instructions: Vec<(usize, &metatape::program::Instruction)> = program
        .instructions
        .iter()
        .map(|(_, instruction)| instruction)
        .enumerate()
        .collect();

    println!("{:#?}", instructions);
    Ok(())
}
