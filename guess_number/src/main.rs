use std::io;

fn main() {
    println!("Hello, world!");

    let mut guess: String = String::new();
    io::stdin().read_line(&mut guess).expect("error");

    println!("{}", guess);
}
