mod locale;

use locale::{localize, IS_ENGLISH, Locale, HELLO};
use std::fmt::Display;

// Simple printing helper for demonstration purposes
fn print_columns<T>(first_column: T, second_column: T)
where
    T: Display,
{
    println!("{:^10} {:^10}", first_column, second_column);
}

fn main() {
    // Print locale labels
    print_columns(Locale::LABELS[0], Locale::LABELS[1]);

    // Print localized values
    print_columns(localize!(HELLO, Locale::En), localize!(HELLO, Locale::Ru));
    print_columns(
        localize!(IS_ENGLISH, Locale::En),
        localize!(IS_ENGLISH, Locale::Ru),
    );
}
