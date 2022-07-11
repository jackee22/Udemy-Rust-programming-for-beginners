// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

enum Color {
  Yellow,
  White,
  Black,
}

fn print_color(my_color: Color) {
  match my_color {
    Color::Yellow => println!("Yellow"),
    Color::White => println!("White"),
    Color::Black => println!("Black"),
    _ => println!("Nothing")
  }
}

fn main() {
  print_color(Color::Black);
}
