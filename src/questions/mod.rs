#[cfg(feature = "email")]
pub use email::*;
pub use input::*;
pub use integer::*;
pub use select::*;

use console::style;

#[cfg(feature = "email")]
mod email;
mod input;
mod integer;
mod select;

const ARROW: char = '❯';

fn formatted_question(title: String, actions: &[(&str, &str)]) -> String {
    format!(
        "{} {} ({})",
        style('?').green(),
        style(title).bold(),
        actions
            .iter()
            .map(|(key, action)| format!("{} to {action}", style(format!("<{key}>")).red()))
            .collect::<Vec<_>>()
            .join(", ")
    )
}

fn formatted_answered_question(title: String, answer: String) -> String {
    format!(
        "{} {} {}",
        style('?').green(),
        style(title).bold(),
        style(answer).dim(),
    )
}
