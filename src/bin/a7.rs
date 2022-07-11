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
  Black,
  Green,
  White,
}

fn color_print(my_color: Color) {
  match my_color {
    Color::Black => println!("B"),
    Color::Green => println!("G"),
    Color::White => println!("W"),
  }
}

fn main() {
  color_print(Color::Black)
}
