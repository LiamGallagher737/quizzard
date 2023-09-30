use console::Term;
use quizzard::{Select, SelectEnum};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let term = Term::stdout();
    term.hide_cursor()?;

    let answer = Select::<Food>::new("Which of these is your favourite?").ask(&term)?;
    println!("You answered Food::{answer:?}");

    term.show_cursor()?;
    Ok(())
}

#[derive(SelectEnum, Debug)]
enum Food {
    Pizza,
    Cake,
    #[prompt("Hot Dog")]
    HotDog,
    Burger,
    Fries,
    Tacos,
    Sushi,
    Pasta,
    Salad,
    Soup,
    Sandwich,
    Steak,
    Chicken,
    Rice,
    Chips,
    #[prompt("Ice Cream")]
    IceCream,
    Yogurt,
    Cereal,
    Eggs,
    Bacon,
    Waffles,
    Pancakes,
    #[prompt("French Toast")]
    FrenchToast,
    Oatmeal,
    Granola,
    Smoothie,
    Sausage,
    Muffin,
    Bagel,
    Donut,
    Nachos,
    Hummus,
    Ramen,
    Pho,
    #[prompt("Pad Thai")]
    PadThai,
    Dumplings,
    Risotto,
}
