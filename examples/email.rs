use console::Term;
use quizzard::Email;
use std::error::Error;

// The `Email` input requires the "email" feature of quizzard to be enabled

fn main() -> Result<(), Box<dyn Error>> {
    let term = Term::stdout();

    let email = Email::new("What's your email?").ask(&term)?;

    println!("You're email is {email}");

    Ok(())
}
