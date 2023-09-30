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
        term.write_line(&format!(
            "{} {} ({} to select)",
            style('?').green(),
            style(&self.title).bold(),
            style("<enter>").red(),
        ))?;

        let mut selected = 0;
        loop {
            for (n, variant) in T::VARIANTS.iter().enumerate() {
                let prompt = variant.prompt();
                let arm = if n == selected {
                    format!("{} {}", style(ARROW).red(), style(prompt).red().bold())
                } else {
                    format!("  {}", prompt)
                };
                term.write_line(&arm)?;
            }
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
                    term.clear_last_lines(T::VARIANTS.len())?;
                    break;
                }
            }
        }
    }
}
