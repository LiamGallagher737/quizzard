use crate::questions::ARROW;
use crate::Error::Other;
use crate::Result;
use console::{style, Key, Term};

pub trait SelectEnum: Sized + 'static {
    const VARIANTS: &'static [Self];
    fn prompt(&self) -> &'static str;
    fn to_index(&self) -> usize;
    fn from_index(n: usize) -> Option<Self>;
}

/// Get a single enum variant input from the user
///
/// # Example
/// ```no_run
/// use console::Term;
/// use quizzard::{Select, SelectEnum};
///
/// #[derive(SelectEnum, Debug)]
/// enum Speed {
///     Slow,
///     Medium,
///     Fast,
/// }
///
/// # fn main() -> Result<(), quizzard::Error> {
/// let term = Term::stdout();
/// let answer = Select::<Speed>::new("How fast is your code?").ask(&term)?;
/// println!("You answered Speed::{answer:?}");
/// # Ok(())
/// # }
/// ```
#[derive(Default)]
pub struct Select<T: SelectEnum> {
    title: String,
    initial: Option<T>,
}

impl<T: SelectEnum> Select<T> {
    /// Creates a select with the given title
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            initial: None,
        }
    }

    /// Set the initially selected variant
    pub fn initial(mut self, initial: T) -> Self {
        self.initial = Some(initial);
        self
    }

    /// Ask the question getting the selected enum variant as a result
    pub fn ask(&self, term: &Term) -> Result<T> {
        let mut selected = self
            .initial
            .as_ref()
            .map(|v| v.to_index())
            .unwrap_or_default();
        loop {
            term.write_line(&format!(
                "{} {} ({} to select)",
                style('?').green(),
                style(&self.title).bold(),
                style("<enter>").red(),
            ))?;

            Self::write_options(term, selected)?;

            loop {
                let key = term.read_key()?;
                let rerender = match key {
                    Key::Enter => {
                        term.clear_last_lines(T::VARIANTS.len() + 1)?;
                        let ans = T::from_index(selected).ok_or(Other("Index out of range"))?;
                        term.write_line(&format!(
                            "{} {} {}",
                            style('?').green(),
                            style(&self.title).bold(),
                            style(ans.prompt()).dim(),
                        ))?;
                        return Ok(ans);
                    }
                    Key::ArrowUp => {
                        if selected > 0 {
                            selected -= 1
                        } else {
                            selected = T::VARIANTS.len() - 1
                        };
                        true
                    }
                    Key::ArrowDown => {
                        if selected < T::VARIANTS.len() - 1 {
                            selected += 1
                        } else {
                            selected = 0
                        };
                        true
                    }
                    Key::Char(c @ '1'..='9') => {
                        let index = c.to_digit(10).unwrap() as usize - 1;
                        if index < T::VARIANTS.len() {
                            selected = index;
                            true
                        } else {
                            false
                        }
                    }
                    _ => false,
                };

                if rerender {
                    let (_width, height) = term.size();
                    let per_page = height as usize - 2;
                    term.clear_last_lines((T::VARIANTS.len() + 1).min(per_page))?;
                    break;
                }
            }
        }
    }

    /// Ask the question optionally getting either the selected enum variant or none as a result
    pub fn ask_opt(&self, term: &Term) -> Result<Option<T>> {
        let mut selected = 0;
        loop {
            term.write_line(&format!(
                "{} {} ({} to select, {} to skip)",
                style('?').green(),
                style(&self.title).bold(),
                style("<space>").red(),
                style("<enter>").red(),
            ))?;

            Self::write_options(term, selected)?;

            loop {
                let key = term.read_key()?;
                let rerender = match key {
                    Key::Char(' ') => {
                        term.clear_last_lines(T::VARIANTS.len() + 1)?;
                        let ans = T::from_index(selected).ok_or(Other("Index out of range"))?;
                        term.write_line(&format!(
                            "{} {} {}",
                            style('?').green(),
                            style(&self.title).bold(),
                            style(ans.prompt()).dim(),
                        ))?;
                        return Ok(Some(ans));
                    }
                    Key::Enter => {
                        term.clear_last_lines(T::VARIANTS.len() + 1)?;
                        term.write_line(&format!(
                            "{} {} {}",
                            style('?').green(),
                            style(&self.title).bold(),
                            style("Skipped").dim(),
                        ))?;
                        return Ok(None);
                    }
                    Key::ArrowUp => {
                        if selected > 0 {
                            selected -= 1
                        } else {
                            selected = T::VARIANTS.len() - 1
                        };
                        true
                    }
                    Key::ArrowDown => {
                        if selected < T::VARIANTS.len() - 1 {
                            selected += 1
                        } else {
                            selected = 0
                        };
                        true
                    }
                    _ => false,
                };

                if rerender {
                    let (_width, height) = term.size();
                    let per_page = height as usize - 2;
                    term.clear_last_lines((T::VARIANTS.len() + 1).min(per_page))?;
                    break;
                }
            }
        }
    }

    fn write_options(term: &Term, selected: usize) -> Result<()> {
        let (rows, _) = term.size();
        let per_page = rows as usize - 2;

        let page = page(term, selected);
        let start = page * per_page;
        let end = start + per_page;

        for (n, variant) in T::VARIANTS.iter().enumerate().skip(start).take(per_page) {
            if n >= end {
                break;
            }

            let prompt = variant.prompt();
            let arm = if n == selected {
                format!("{} {}", style(ARROW).red(), style(prompt).red().bold())
            } else {
                format!("  {}", prompt)
            };
            term.write_line(&arm)?;
        }
        Ok(())
    }
}

fn page(term: &Term, selected: usize) -> usize {
    let (rows, _) = term.size();
    let per_page = rows as usize - 2;
    selected / per_page
}
