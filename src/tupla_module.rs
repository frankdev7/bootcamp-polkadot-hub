pub mod tupla_module {
  pub fn tupla_operations() {
    let x:(i32, &str) = (1, "hello");
    let (a, b) = x;
    println!("a is {}", a);
    println!("b is {}", b);
  }
}