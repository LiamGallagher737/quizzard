#![cfg(feature = "email")]

use crate::{Input, Result};
use console::Term;
use email_address::{EmailAddress, Error};
use std::result;
use std::str::FromStr;

/// Get an email input from the user
///
/// # Example
/// ```no_run
/// use console::Term;
/// use quizzard::Email;
///
/// # fn main() -> Result<(), quizzard::Error> {
/// let term = Term::stdout();
/// let email = Email::new("How old are you?")
///     .ask(&term)?;
/// println!("You're email is {email}");
/// # Ok(())
/// # }
/// ```
pub struct Email {
    title: String,
}

impl Email {
    /// Creates an email input with the given title
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
        }
    }

    /// Ask the question getting the inputted email as a result
    pub fn ask(&self, term: &Term) -> Result<EmailAddress> {
        Input::new(self.title.clone())
            .validator(Self::validator)
            .ask(term)
    }

    fn validator(input: String) -> result::Result<EmailAddress, String> {
        let res = EmailAddress::from_str(&input);
        match res {
            Ok(ans) => Ok(ans),
            Err(err) => Err(match err {
                Error::InvalidCharacter => "An invalid character is present",
                Error::MissingSeparator => "Missing @ separator",
                Error::LocalPartEmpty => "The local part is empty",
                Error::LocalPartTooLong => "The local part is too long",
                Error::DomainEmpty => "The domain is empty",
                Error::DomainTooLong => "The domain is too long",
                Error::SubDomainTooLong => "The subdomain is too long",
                Error::DomainTooFew => "Not enough sub domains are present",
                Error::DomainInvalidSeparator => "Invalid placement of a '.'",
                Error::UnbalancedQuotes => "Unbalanced quotes",
                Error::InvalidComment => "An invalid comment is present",
                Error::InvalidIPAddress => "Invalid IP address",
            }
            .to_string()),
        }
    }
}
