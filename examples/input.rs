use console::Term;
use quizzard::Text;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let term = Term::stdout();

    let answer = Text::new("What's your name?").ask(&term)?;
    println!("You answered {answer:?}");

    Ok(())
}
