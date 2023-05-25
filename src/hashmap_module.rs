pub mod hashmap_module {
    use std::collections::HashMap;

  pub fn hasmap_1() {
    let mut contacts = HashMap::new();
    contacts.insert("Daniel", "798-1364");
    contacts.insert("Ashley", "645-7689");
    contacts.insert("Robert", "798-8291");

    for (contact, &number) in contacts.iter() {
      println!("Calling {}: {}", contact, call(number));
    }

    match contacts.get("Daniel") {
      Some(number) => println!("Calling Daniel: - {}", number),
      _ => println!("Don't have Daniel's number")
    }

    contacts.insert("Daniela", "164-6743");

    match  contacts.get("Ashley") {
        Some(number) => println!("Calling Ashley: -"),
        _ => println!("Don't have Ashley's number.")
    }

    for( key, value) in contacts.iter() {
      println!("Calling - {} -> {}", &key, &value);
    }

  }

  fn call(number: &str) -> &str {
    match number {
      "798-1364" => "We're sorry, the call cannot be completed as dialed. Please hang up and try again",
      "645-7689" => "Hello, this is Mr. Awesome's Pizza. My name is Fred. What can I get for you today?",
      _ => "Hi! Who is this again?"
    }
  }

  // HashMap with types and structs

  #[derive(PartialEq, Eq, Hash)]
  struct Account<'a> {
    username: &'a str,
    password: &'a str,
  }

  struct AccountInfo<'a> {
    name: &'a str,
    email: &'a str,
  }

  type Accounts<'a> = HashMap<Account<'a>, AccountInfo<'a>>;

  fn try_logon<'a>(accounts: &Accounts<'a>, username: &'a str, password: &'a str) {
    println!("Username: {}", username);
    println!("Password: {}", password);
    println!("Attempting logon...");

    let logon: Account = Account { username, password };
    
    match accounts.get(&logon) {
      Some(account_info) => {
        println!("Successful logon!");
        println!("Name: {}", account_info.name);
        println!("Email: {}", account_info.email);
      }
      _ => println!("Login failed!")
    }
  }

  pub fn login() {
    let mut accounts: Accounts = HashMap::new();

    let account: Account = Account { username: "myuser123", password: "mypass123" };
    let account_info: AccountInfo = AccountInfo { name: "frank", email: "frank@mail.com" };

    accounts.insert(account, account_info);

    try_logon(&accounts, "myfaileduser", "myfailedpassword");
    try_logon(&accounts, "myuser123", "mypass123");

  }
}