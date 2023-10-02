use console::Term;
use quizzard::{MultiSelect, Select, SelectEnum};
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

#[derive(SelectEnum, Debug)]
enum Language {
    Rust,
    Go,
    C,
    #[prompt("C++")]
    Cpp,
    #[prompt("JavaScript 🤮")]
    JavaScript,
}

fn main() -> Result<(), Box<dyn Error>> {
    let term = Term::stdout();
    term.hide_cursor()?;

    let answer = Select::<Speed>::new("How fast is your code?").ask(&term)?;
    println!("You answered Speed::{answer:?}");

    let answer = Select::<Speed>::new("How fast is your code? (optional)").ask_opt(&term)?;
    println!("You optionally answered {answer:?}");

    let answers = MultiSelect::<Language>::new("What languages to you use?")
        .min(2)
        .max(4)
        .ask(&term)?;

    println!("You selected: {answers:?}");

    term.show_cursor()?;
    Ok(())
}
