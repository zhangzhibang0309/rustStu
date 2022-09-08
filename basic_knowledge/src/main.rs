fn main() {
    // 1.变量相关知识
    // 变量
    // let x: i32 = 5;
    // println!("x的值是 {}", x);
    // let x: &str = "六";
    // println!("x的值是 {}", x);
    // 常量
    // const MONEY: u32 = 100_000;
    // println!("{}", MONEY);

    // 2.类型相关知识
    // 标量

    // 整数类型
    // let a: i32 = 90_000; // 十进制
    // let b: i32 = 0x100; // 十六进制
    // let c: i32 = 0o11; // 八进制
    // let d: i32 = 0b11; // 二进制
    // 整数溢出问题
    // let f: u8 = 256; 八位无符号0-255，256溢出
    // println!("{}", a);
    // println!("{}", b);
    // println!("{}", c);
    // println!("{}", d);
    // 浮点数类型
    // let f1: f32 = 1.11;
    // let f2: f64 = 10.232323444;
    // 布尔类型
    // let b1: bool = true;
    // let b2: bool = false;
    // 字符类型
    // let ch1: char = '1';
    // let cat: char = '🐱';
    // let hhh: char = '😄';
    // println!("{}{}{}", ch1, cat, hhh);

    // 复合类型
    // tuple
    // let tup = ("hhhh", 22);
    // let (channel, age) = tup;
    // let chan = tup.1;
    // println!("{} {} {}", channel, age, chan);
    // array
    // let arr: [i32; 3] = [1, 2, 3];
    // let arr2: [i32; 3] = [1; 3];
    // println!("{} {}", arr[0], arr2[2]);

    // 函数的使用
    let n: i32 = sum(1, 2);
    println!("{}", n);
}

fn sum(x: i32, y: i32) -> i32 {
    return x + y;
}
