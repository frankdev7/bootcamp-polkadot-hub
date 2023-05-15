pub mod enum_module {
  enum Language {
    JS,
    GO,
    VB
  }

  pub fn program() {
    let var = Language::GO;
    match var {
      Language::JS => println!("JS in 8 Hours"),
      Language::GO => println!("GO in 8 Hours"),
      Language::VB => println!("VB in 8 Hours"),
    }
  }

  enum Color {
    Red,
    Blue,
    Gree,
    RGB(u32, u32, u32),
    CMYK(u32, u32, u32, u32)
  }

  pub fn colorToString() {
    let mut textColor = String::from("Color: ");
    let color: Color = Color::RGB(20, 30, 40);
    let rgb_string: String;

    let c: &str = match color {
      Color::RGB(20,y,z) => {
        rgb_string = format!("r = {x}, g = {y}, b = {z}", x = 20, y = y, z = z);
        &rgb_string
      },
      Color::Blue => "AZUL",
      _ => "."
    };
    textColor.push_str(c);
    println!("{:?}",textColor);
  }
}