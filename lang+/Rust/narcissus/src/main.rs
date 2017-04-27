use std::io;

// 从键盘输入一个数
// 判断这个数是不是水仙花数

fn main() {

    let mut s = String::new();

    io::stdin().read_line(&mut s);

    let i = match s.trim().parse::<u32>() {
        Ok(data) => data,
        Err(e) =>  {
            println!("{}", e);
            0
        },
    };

    if i == 0 {
        println!("请输入一个正整数");

        return;
    }

    if i < 10 {
        println!("{} 是水仙花数", i);

        return;
    }

    let len = s.trim().len() as u32;

    let mut sum: u32 = 0;

    for index in 0..s.trim().len() {
        let x = s.chars().nth(index).unwrap().to_digit(10);

        match x {
            Some(data) => {
               sum += (data as u32).pow(len);
            },
            None => {},
        }

    }

    println!("{} ... {}", i, sum);

    if i == sum {
        println!("{} 是水仙花数", i);
    }
}
