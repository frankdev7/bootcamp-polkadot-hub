pub mod struct_1_module {
  #[derive(Debug)]
  pub struct PersonBase {
    name: String,
    surname: String,
    age: u32
  }

  pub fn struct_custom1() {

    let me = PersonBase {
      name: String::from("Frank"),
      surname: String::from("Rojas"),
      age: 26_u32
    };
    println!("Name: {}, Surname: {}", me.name, me.surname);
    println!("{:?}", me);
    // let Person(name, surname) = me;


  }

  pub fn struct_custom2() {
    struct Job(i32, String);
    let apiJob = Job(15, String::from("crypto"));
    println!("Job values: {:?} y {:?}", apiJob.0, apiJob.1);
  }


  pub fn struct_custom3() {
    let p: Person = Person {
      name: String::from("frank"),
      surname: String::from("Rojas"),
      age: 26_u16,
      address: Address {
        street: String::from(""),
        number: 1,
        province: String::from(""),
        postal_code: String::from("")
      },
    };
    println!("{:?}", p);
  }

  #[derive(Debug)]
  pub struct Carro {
    make: String,
    model: String,
    license_plate: str,
  }

  #[derive(Debug)]
  pub struct Address {
    street: String,
    number: u64,
    province: String,
    postal_code: String,
  }

  #[derive(Debug)]
  pub struct Person {
    surname: String,
    name: String,
    age: u16,
    address: Address
  }

}

pub mod struct_2_module {
  #[derive(Debug)]
  struct Address<'b> {
    street: &'b mut String,
    number: &'b mut u64,
    province: &'b mut String,
    postal_code: &'b mut String,
  }

  #[derive(Debug)]
  struct Person<'a> {
    name: &'a mut String,
    surname: &'a mut String,
    age: &'a mut u16,
    address: &'a mut Address<'a>,
  }

  pub fn struct_2_custom() {
    let p: Person = Person { 
      name: &mut String::from("Frank"), 
      surname: (&mut String::from("Rojas")), 
      age: &mut 26_u16, 
      address: &mut Address {
        street: &mut String::from(""), 
        number: &mut 1, 
        province: &mut String::from(""), 
        postal_code: &mut String::from("") 
      }
    };

    println!("{:?}", p);
    p.surname.push_str(" Cc");
    println!("{:?}", p);
    {  
      let mut postal_code = String::from("70001");
      p.address.postal_code = &mut postal_code;
      println!("{:?}", p);
    }
    
  }
}