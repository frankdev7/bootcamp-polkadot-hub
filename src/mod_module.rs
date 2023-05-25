pub mod my_mod {
  pub mod nested {
    pub(super) fn my_nested_function() {
      println!("hello from my function");
    }
  }

  pub fn my_function() {
    nested::my_nested_function();
  }

  pub fn call_my_other_function() {
    self::my_function();
  }

  pub fn call_my_external_function() {
    super::my_external_function();
  }
}

fn my_external_function() {
  println!("hello from my external function");
}