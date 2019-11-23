use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!("我会在1到100想一个数，看你几轮可以猜中它！ :)");

    let mut round_num = 0;
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        round_num += 1;
        println!("这是你的第{}次尝试。", round_num);
        println!("请输入你的猜测：");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("读取标准输出失败");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("你得输入数字！");
                continue;
            }
        };

        println!("你猜的数字是{}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("太小了！ :P"),
            Ordering::Greater => println!("太大了！ :P"),
            Ordering::Equal => {
                println!("恭喜你猜对了！");
                break;
            }
        }
    }
}
