use console::Term;
use quizzard::Text;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let term = Term::stdout();

    let name = Text::new("What's your name?")
        .charset(('A'..='Z').chain('a'..='z'))
        .ask(&term)?;

    let job = Text::new("What's your job title?")
        .default("Developer")
        .ask(&term)?;

    println!("Your name is {name} and your job title is {job}");

    Ok(())
}
