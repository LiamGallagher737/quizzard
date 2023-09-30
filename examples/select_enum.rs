use quizzard::SelectEnum;

#[derive(SelectEnum, Debug)]
enum Speed {
    Slow,
    Medium,
    Fast,
    #[prompt("Really Fast")]
    ReallyFast,
    #[prompt("EXTREMELY FAST!!!")]
    ExtremelyFast,
}

fn main() {
    for variant in Speed::VARIANTS {
        println!("{variant:?} => {}", variant.prompt());
    }
}
