pub mod rs_module {
  pub fn rs_operations() {
    let mut count = 0;
    'counting_up: loop {
      let number = 10;
      loop {
        println!("number = {number}");
        if count == 2 {
          break 'counting_up;
        }
        println!("count = {count}");
        count += 1;
      }
    }
    println!("End count = {count}");
  }
}