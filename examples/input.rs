use console::Term;
use quizzard::Input;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let term = Term::stdout();

    let name = Input::new("What's your name?")
        .charset(('A'..='Z').chain('a'..='z'))
        .validator(|input| {
            if input.len() > 3 {
                Ok(input)
            } else {
                Err("Too Short".into())
            }
        })
        .ask(&term)?;

    let job = Input::new("What's your job title?")
        .default("Developer")
        .validator(|input| Ok(input))
        .ask(&term)?;

    println!("Your name is {name} and your job title is {job}");

    Ok(())
}
