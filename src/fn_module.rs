pub mod fn_module {
  pub fn simple_function(x: i32) -> i32 {
    return x+1;
  }

  pub fn clouseres() {
    let suma_uno = |x: i32| x+1;
    println!("Result: {}", suma_uno(10));

    let suma_dos = |x| {
      let mut result: i32 = x;
      result += 1;
      result += 1;
      result
    };
    println!("Result: {}", suma_dos(20));

    let num = 5;
    let sumar_numero = |x:i32| x + num;
    println!("Result: {}", sumar_numero(20));
  }
}