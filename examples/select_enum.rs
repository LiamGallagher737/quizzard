use console::Term;
use quizzard::{Select, SelectEnum};
use std::error::Error;

#[derive(SelectEnum, Debug)]
enum Speed {
    Slow,
    Medium,
    Fast,
    #[prompt("Really Fast")]
    ReallyFast,
    #[prompt("EXTREMELY FAST!!!")]
    ExtremelyFast,
}

fn main() -> Result<(), Box<dyn Error>> {
    let term = Term::stdout();
    term.hide_cursor()?;

    let answer = Select::<Speed>::new("How fast is your code?").ask(&term)?;
    println!("You answered Speed::{answer:?}");

    let answer = Select::<Speed>::new("How fast is your code? (optional)").ask_opt(&term)?;
    println!("You optionally answered {answer:?}");

    term.show_cursor()?;
    Ok(())
}
