// Struct Declaration
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

fn main() {
    // Creating an instance via functions & accessing its fields
    let midnightblue = get_midnightblue_color();
    println!(
        "Midnight Blue = rgb({}, {}, {})",
        midnightblue.red, midnightblue.green, midnightblue.blue
    ); //Midnight Blue = rgb(25, 25, 112)
}

fn get_midnightblue_color() -> Color {
    Color {
        red: 25,
        green: 25,
        blue: 112,
    }
}
