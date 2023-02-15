// use logger_lib::state_types::*;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(EnumIter, Debug, PartialEq)]
enum Color {
    Red,
    Green { range: usize },
    Blue(usize),
    Yellow,
}


fn main() {
    println!("Attributes");
    for color in Color::iter() {
        println!("My favorite color is {:?}", color);
    }
}