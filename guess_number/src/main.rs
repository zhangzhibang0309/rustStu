use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Hello, world!");

    let secret_number: i32 = rand::thread_rng().gen_range(1..=100);

    println!("secret: {}", secret_number);

    loop {
        println!("请输入你要猜测的数字：");
        let mut guess: String = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("error: 读取用户输入失败");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("请输入数字！！");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("太小"),
            Ordering::Greater => println!("太大"),
            Ordering::Equal => {
                print!("你猜对了");
                break;
            }
        }
    }
}
