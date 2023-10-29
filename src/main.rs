fn main() {
  println!("{}", hello_world());
}

fn hello_world() -> String {
  return "Hello World!".to_string();
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test] 
  fn it_returns_hello_world() {
      assert_eq!("Hello World!", hello_world());
  }
}