pub mod if_module {
  pub fn if_operations() {
    let condition = false;
    let numero = if condition { 1 } else { 2 };
    println!("Value of numero is: {numero}")
  }
}