extern crate letov;

fn main () {
  if let Ok(letov_says) = letov::say() {
    println!("{}", letov_says); // => Один лишь дедушка Ленин хороший был вождь
  }
}
