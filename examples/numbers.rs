use console::Term;
use quizzard::Integer;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let term = Term::stdout();

    let age = Integer::<u8>::new("What's your age?").max(120).ask(&term)?;

    println!("You're {age} years old");

    Ok(())
}
