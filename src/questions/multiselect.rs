use crate::questions::{formatted_answered_question, formatted_question, FILLED_DOT, OUTLINE_DOT};
use crate::Result;
use crate::SelectEnum;
use console::{style, Key, Term};

/// Get multiple enum variants input from the user
///
/// # Example
/// ```no_run
/// use console::Term;
/// use quizzard::{MultiSelect, SelectEnum};
///
/// #[derive(SelectEnum, Debug)]
/// enum Language {
///     Rust,
///     Go,
///     C,
/// }
///
/// # fn main() -> Result<(), quizzard::Error> {
/// let term = Term::stdout();
/// let answers = MultiSelect::<Language>::new("What languages do you use?").ask(&term)?;
/// println!("You selected {answers:?}");
/// # Ok(())
/// # }
/// ```
pub struct MultiSelect<T: SelectEnum> {
    title: String,
    initial: Vec<T>,
    min: usize,
    max: usize,
}

impl<T: SelectEnum> MultiSelect<T> {
    /// Creates a select with the given title
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            initial: Vec::new(),
            min: 0,
            max: usize::MAX,
        }
    }

    /// Set the initially selected variant
    pub fn initial(mut self, initial: Vec<T>) -> Self {
        self.initial = initial;
        self
    }

    /// Set the minimum variants allowed to be selected
    pub fn min(mut self, min: usize) -> Self {
        self.min = min;
        self
    }

    /// Set the maximum variants allowed to be selected
    pub fn max(mut self, max: usize) -> Self {
        self.max = max;
        self
    }

    /// Ask the question getting a list of the selected enum variants as a result
    pub fn ask(self, term: &Term) -> Result<Vec<T>> {
        let mut cursor = 0;
        let mut selected = self.initial;
        let mut active_err_msg = false;
        term.write_line(&formatted_question(
            self.title.clone(),
            &[("space", "select"), ("enter", "proceed")],
        ))?;
        loop {
            Self::write_options(term, cursor, &selected)?;

            loop {
                let key = term.read_key()?;
                let rerender = match key {
                    Key::Char(' ') => {
                        let variant = T::from_index(cursor).unwrap();
                        if let Some(n) = selected
                            .iter()
                            .position(|v| v.to_index() == variant.to_index())
                        {
                            selected.remove(n);
                        } else {
                            selected.push(variant);
                        }
                        true
                    }
                    Key::Enter => {
                        if selected.len() < self.min || selected.len() > self.max {
                            Self::clear_render(term)?;
                            if active_err_msg {
                                term.clear_last_lines(1)?;
                            }
                            term.write_line(&format!(
                                "{} {}",
                                style('X').red(),
                                style(if selected.len() < self.min {
                                    format!("Must select at least {}", self.min)
                                } else {
                                    format!("Must select {} or less", self.max)
                                })
                                .red()
                            ))?;
                            active_err_msg = true;
                            break;
                        }
                        term.clear_last_lines(T::VARIANTS.len() + 1)?;
                        term.write_line(&formatted_answered_question(
                            self.title.clone(),
                            if !selected.is_empty() {
                                selected
                                    .iter()
                                    .map(|v| v.prompt())
                                    .collect::<Vec<_>>()
                                    .join(", ")
                            } else {
                                "Skipped".to_string()
                            },
                        ))?;
                        return Ok(selected);
                    }
                    Key::ArrowUp => {
                        if cursor > 0 {
                            cursor -= 1
                        } else {
                            cursor = T::VARIANTS.len() - 1
                        };
                        true
                    }
                    Key::ArrowDown => {
                        if cursor < T::VARIANTS.len() - 1 {
                            cursor += 1
                        } else {
                            cursor = 0;
                        };
                        true
                    }
                    Key::Char(c @ '1'..='9') => {
                        let index = c.to_digit(10).unwrap() as usize - 1;
                        if index < T::VARIANTS.len() {
                            cursor = index;
                            true
                        } else {
                            false
                        }
                    }
                    _ => false,
                };

                if rerender {
                    Self::clear_render(term)?;
                    break;
                }
            }
        }
    }

    fn write_options(term: &Term, cursor: usize, selected: &[T]) -> Result<()> {
        let (rows, _) = term.size();
        let per_page = rows as usize - 2;

        let page = page(term, cursor);
        let start = page * per_page;
        let end = start + per_page;

        let selected_indices: Vec<usize> = selected.iter().map(|v| v.to_index()).collect();

        for (n, variant) in T::VARIANTS.iter().enumerate().skip(start).take(per_page) {
            if n >= end {
                break;
            }

            let prompt = variant.prompt();
            let dot = if selected_indices.contains(&n) {
                FILLED_DOT
            } else {
                OUTLINE_DOT
            };
            let arm = if cursor == n {
                format!("{} {}", style(dot).red(), style(prompt).red().bold())
            } else {
                format!("{dot} {}", prompt)
            };
            term.write_line(&arm)?;
        }
        Ok(())
    }

    fn clear_render(term: &Term) -> Result<()> {
        let (_width, height) = term.size();
        let per_page = height as usize;
        term.clear_last_lines((T::VARIANTS.len()).min(per_page))?;
        Ok(())
    }
}

fn page(term: &Term, selected: usize) -> usize {
    let (rows, _) = term.size();
    let per_page = rows as usize - 2;
    selected / per_page
}
