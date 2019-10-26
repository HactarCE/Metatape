#[macro_use]
extern crate pest_derive;

mod metatape;

fn main() {
    println!("Hello, world!");
    println!(
        "{:#?}",
        metatape::parser::parse(
            "
ex<
!h !e !l !l !o !_ !w !o !r !l !d !!


@ _ { oo>o<ooooo }
@ ! { oo>o<oooo>o< }
@ d { o>oo<oo>o<oo }
@ e { o>oo<oo>o<o>o< }
@ h { o>o<oo>o<ooo }
@ l { o>oo<o>oo<oo }
@ o { o>oo<o>oooo< }
@ r { o>ooo<oo>o<o }
@ w { o>ooo<o>ooo< }
            "
        )
    );
}
