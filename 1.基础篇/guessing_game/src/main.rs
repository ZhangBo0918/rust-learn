use rand::Rng;
use std::io;
fn main() {
    println!("请猜一猜数字:");
    let mut guess = String::new();
    let secret_number = rand::thread_rng().gen_range(0..101);
    io::stdin().read_line(&mut guess).expect("读取失败");
    print!("你猜的是：{}", guess);
    print!("要猜的是:{}", secret_number);
}
