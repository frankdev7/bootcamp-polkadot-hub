pub mod vector_module {
  pub fn vector_1() {
    let collected_iterator: Vec<i32> = (0..10).collect();
    println!("Collected (0..10) into: {:?}", collected_iterator);
    // collected_iterator.push(0);
  }

  pub fn vector_2() {
    let mut xs = vec![1i32, 2, 3];
    println!("Initial vector: {:?}", xs);
    
    xs.push(4);
    println!("Initial vector: {:?}", xs);
    
    println!("Vector length: {}", xs.len());
    
    println!("Second element: {}", xs[1]);
    
    println!("Pop last element: {:?}", xs.pop());
    println!("Initial vector: {:?}", xs);
  }

  pub fn vector_3_iter() {
    let xs = vec![1,2,3];

    for x in xs.iter() {
      print!(" {}", x);
    }
  }

  pub fn vector_3_enumerate() {
    let xs = vec![1,2,3];
    for(i,x) in xs.iter().enumerate() {
      println!("In position {} we have value {}", i,x);
    }
  }

  pub fn vector_4_iter_mut() {
    let mut xs = vec![1,2,3];
    for x in xs.iter_mut() {
      *x *= 3;
    }
    println!("Updated vector {:?}", xs)
  }
}