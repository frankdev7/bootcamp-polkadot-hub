pub mod ownership_borrowing_module {
  pub fn borrowing() {
    let value_string: String = String::from("hi world!");
    let new_value: String = fn_parameter_string(&value_string);
    println!("Value: {}", value_string);
    println!("Value: {}", new_value);
  }

  fn fn_parameter_string(p: &String) -> String {
    let new_string: String = String::from(p.as_str());
    return new_string.to_uppercase()
  }

  pub fn ownership() {
    let x = String::from("try");
    let y = x;
    // println!("{}", x);  ERROR because x doesn't own the "try" value

    let value = String::from("Hi");
    ownership_with_error(value);
    // println!("Value: {}", value);

  }

  fn ownership_with_error(valueFn: String) {
    println!("Borrowing: {}", valueFn);
  }

  pub fn borrowing_mutable_i32() {
    let mut x = 5;
    {
      let y = &mut x;
      *y += 1;
    }
    println!("{}", x);
  }

  pub fn borrowing_mutable_string() {
    let mut s = String::from("Rust");
    modificar_string(&mut s);
    println!("Value of the string is: {}", s);
  }

  fn modificar_string(s: &mut String) {
    s.push_str("En Espaniol!");
  }

  pub fn fn_reference_primitive_type_box() {
    let mut valueBox: Box<u64> = Box::<u64>::new(35_u64);
    let valueReturnedByBox: u64 = reference_primitive_type_box(&mut valueBox);
    println!("Value Box: {}", valueReturnedByBox);
  }

  fn reference_primitive_type_box(p: &mut Box<u64>) -> u64 {
    println!("P: {:?}", p as *mut _); // Print the pointer
    println!("P: {}", *p);            // Print the value of the pointer
    println!("P: {}", *p.as_ref());   // Print the value of the pointer but before converts Box<u64> into a reference to a u64
    *p.as_ref() * 2_u64
  }

}