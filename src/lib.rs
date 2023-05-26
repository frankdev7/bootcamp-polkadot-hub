pub trait Summary {
  fn summarize_author(&self) -> String {
    String::from("(Author Info ...)")
  }
  
  fn summarize(&self) -> String {
    String::from("(Read more...)")
  }
}

pub struct NewsArticle {
  pub headline: String,
  pub location: String,
  pub author: String,
  pub content: String,
}

impl Summary for NewsArticle {
  fn summarize(&self) -> String {
      format!("{}, by {} ({})", self.headline, self.author, self.location)
  }
}

pub struct Tweet {
  pub username: String,
  pub content: String,
  pub reply: bool,
  pub retweet: bool,
}

impl Summary for Tweet {
  fn summarize(&self) -> String {
      format!("@{}", self.username)
  }

  fn summarize_author(&self) -> String {
      format!("{}: {}", self.username, self.content)
  }
}

pub fn notify(item: &impl Summary) {
  println!("Breaking news! {}", item.summarize());
}

pub fn returns_summarizable() -> impl Summary {
  NewsArticle {
    headline: String::from("Penguins win"),
    location: String::from("Pittsburgh, PA, USA"),
    author: String::from("Iceburgh"),
    content: String::from("The Pittsburgh")
  }
}