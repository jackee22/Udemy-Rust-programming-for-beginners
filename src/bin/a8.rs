// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

enum Flavour {
  Sweet,
  Fruity,
  Spicy,
}

struct Drink {
  flavour: Flavour,
  fluid_oz: f64,
}

fn print_drink(drink: Drink) {
  match drink.flavour {
    Flavour::Sweet => println!("Sweet"),
    Flavour::Fruity => println!("Fruity"),
    Flavour::Spicy => println!("Spicy"),
  }
}
fn main() {}
