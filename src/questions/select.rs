use crate::questions::ARROW;
use crate::Error::Other;
use crate::Result;
use console::{style, Key, Term};
use std::marker::PhantomData;

pub trait SelectEnum: Sized + 'static {
    const VARIANTS: &'static [Self];
    fn prompt(&self) -> &'static str;
    fn to_index(&self) -> usize;
    fn from_index(n: usize) -> Option<Self>;
}

#[derive(Default)]
pub struct Select<T: SelectEnum> {
    title: String,
    data: PhantomData<T>,
}

impl<T: SelectEnum> Select<T> {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            data: PhantomData::<T>,
        }
    }

    pub fn ask(&self, term: &Term) -> Result<T> {
        let mut selected = 0;
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
                    _ => false,
                };

                if rerender {
                    let (_width, height) = term.size();
                    let per_page = height as usize - 2;
                    term.clear_last_lines(T::VARIANTS.len().min(per_page))?;
                    break;
                }
            }
        }
    }

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
                    term.clear_last_lines(T::VARIANTS.len().min(per_page))?;
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
