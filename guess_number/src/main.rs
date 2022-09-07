use rand::Rng;
use std::{io, cmp::Ordering};

fn main() {
    println!("Hello, world!");

    let secret_number: i32 = rand::thread_rng().gen_range(1..=100);
    println!("secret: {}", secret_number);

    let mut guess: String = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("error: 读取用户输入失败");

    println!("guess: {}", guess);

    let guess: i32 = guess.trim().parse().expect("error: 请输入数字");

    match guess.cmp(&secret_number) {
        Ordering::Less => print!("太小"),
        Ordering::Greater => print!("太大"),
        Ordering::Equal => print!("你猜对了"),
    }
}
