use crate::{Input, Result};
use console::Term;
use num_traits::{Bounded, Zero};
use std::fmt::Display;
use std::num::{IntErrorKind, ParseIntError};
use std::result;
use std::str::FromStr;

/// Get an integer input from the user
///
/// # Example
/// ```no_run
/// use console::Term;
/// use quizzard::Integer;
///
/// # fn main() -> Result<(), quizzard::Error> {
/// let term = Term::stdout();
/// // The charset allows you to only allow certain characters
/// // The following will only allow english characters
/// let answer = Integer::<u8>::new("How old are you?")
///     .max(120)
///     .ask(&term)?;
/// println!("You're {answer} years old");
/// # Ok(())
/// # }
/// ```
pub struct Integer<T: FromStr + Bounded + Zero + PartialOrd + Display + Copy + 'static>
where
    T::Err: IntError,
{
    title: String,
    min: T,
    max: T,
}

impl<T: FromStr + Bounded + Zero + PartialOrd + Display + Copy + 'static> Integer<T>
where
    T::Err: IntError,
{
    /// Creates an integer input with the given title
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            min: T::min_value(),
            max: T::max_value(),
        }
    }

    /// Sets the min value accepted by the input
    pub fn min(mut self, min: T) -> Self {
        self.min = min;
        self
    }

    /// Sets the max value accepted by the input
    pub fn max(mut self, max: T) -> Self {
        self.max = max;
        self
    }

    /// Ask the question getting the inputted integer as a result
    pub fn ask(&self, term: &Term) -> Result<T> {
        let min = self.min;
        let max = self.max;
        Input::new(self.title.clone())
            .charset(self.charset())
            .validator(move |input| Self::validator(input, min, max))
            .ask(term)
    }

    fn validator(input: String, min: T, max: T) -> result::Result<T, String> {
        let res = input.parse::<T>();
        match res {
            Ok(ans) => {
                if ans < min {
                    return Err(format!("Too small! Must be above or equal to {}", min));
                }
                if ans > max {
                    return Err(format!("Too big! Must be below or equal to {}", max));
                }
                Ok(ans)
            }
            Err(err) => Err(match err.kind() {
                IntErrorKind::Empty => "You must enter a value".to_string(),
                IntErrorKind::InvalidDigit => "An invalid character is present".to_string(),
                IntErrorKind::PosOverflow => format!("Too big! Must be below or equal to {}", max),
                IntErrorKind::NegOverflow => {
                    format!("Too small! Must be below or equal to {}", min)
                }
                IntErrorKind::Zero => "Number can't be zero".to_string(),
                _ => "Unable to convert input to number".to_string(),
            }),
        }
    }

    fn charset(&self) -> Vec<char> {
        let mut charset: Vec<char> = ('0'..='9').collect();
        if self.min < T::zero() {
            charset.push('-');
        }
        charset
    }
}

pub trait IntError {
    fn kind(&self) -> &IntErrorKind;
}

impl IntError for ParseIntError {
    fn kind(&self) -> &IntErrorKind {
        self.kind()
    }
}
