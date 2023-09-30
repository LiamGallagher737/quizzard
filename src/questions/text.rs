use crate::questions::{formatted_answered_question, formatted_question, ARROW};
use crate::Result;
use console::{style, Key, Term};

/// Get a single enum variant input from the user
///
/// # Example
/// ```no_run
/// use console::Term;
/// use quizzard::Text;
///
/// # fn main() -> Result<(), quizzard::Error> {
/// let term = Term::stdout();
/// // The charset allows you to only allow certain characters
/// // The following will only allow english characters
/// let answer = Text::new("What is your name?")
///     .charset(('A'..='Z').chain('a'..='z'))
///     .ask(&term)?;
/// println!("You answered {answer}");
/// # Ok(())
/// # }
/// ```
pub struct Text {
    title: String,
    default: Option<String>,
    charset: Option<Vec<char>>,
}

impl Text {
    /// Creates an input with the given title
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            default: None,
            charset: None,
        }
    }

    /// Set the default value of the text value
    pub fn default(mut self, value: impl Into<String>) -> Self {
        self.default = Some(value.into());
        self
    }

    /// Set what characters are allowed to be inputted
    pub fn charset(mut self, value: impl IntoIterator<Item = char>) -> Self {
        self.charset = Some(value.into_iter().collect());
        self
    }

    /// Ask the question getting the inputted string as a result
    pub fn ask(&self, term: &Term) -> Result<String> {
        term.write_line(&formatted_question(
            self.title.clone(),
            &[("enter", "proceed")],
        ))?;

        let mut input = self.default.clone().unwrap_or_default();
        let mut cursor = input.len();

        loop {
            term.write_str(&format!(
                "{} {input}",
                style(ARROW.to_string().repeat(2)).red()
            ))?;

            term.move_cursor_left(100)?;
            term.move_cursor_right(cursor + 3)?;

            loop {
                let key = term.read_key()?;
                let rerender = match key {
                    Key::Char(c) if !c.is_control() => {
                        if let Some(charset) = &self.charset {
                            if !charset.contains(&c) {
                                continue;
                            }
                        }
                        cursor += 1;
                        if cursor == input.len() + 1 {
                            input.push(c);
                            term.write_str(&c.to_string())?;
                            false
                        } else {
                            input.insert(cursor - 1, c);
                            true
                        }
                    }
                    Key::Backspace if !input.is_empty() => {
                        input.remove(cursor - 1);
                        cursor -= 1;
                        if cursor == input.len() {
                            term.clear_chars(1)?;
                            false
                        } else {
                            true
                        }
                    }
                    Key::ArrowLeft if cursor > 0 => {
                        cursor -= 1;
                        term.move_cursor_left(1)?;
                        false
                    }
                    Key::ArrowRight if cursor < input.len() => {
                        cursor += 1;
                        term.move_cursor_right(1)?;
                        false
                    }
                    Key::Enter => {
                        term.clear_line()?;
                        term.clear_last_lines(1)?;
                        term.write_line(&formatted_answered_question(
                            self.title.clone(),
                            input.clone(),
                        ))?;
                        return Ok(input);
                    }

                    _ => false,
                };

                if rerender {
                    term.clear_line()?;
                    break;
                }
            }
        }
    }
}
