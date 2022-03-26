// 功能：1.输入数n，打印fibonacci对应的数 2.输入数n,打印fibonacci数列的前n个数
// 说明：只能打印n<47,超过就非常慢

use std::io;
use std::time::{Duration, Instant};

fn main() {
    println!("Please input a number");

    loop{
        println!("Please input a number to generate fibonacci sequence");
        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Failed to read line");
        
        if input.trim() == "Q" {
            break;
        }

        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("please input a number");
                continue;
            }
        };
        
        println!("number = {}, fibonacci number = {} ", input,  print_fibonacci_array(input));

        let start = Instant::now();
        generate_fibonacci(input);
        let duration = start.elapsed();
        println!("generate_fibonacci duration:{:?}",duration);
    }
}

//打印特定数量的斐波那契数列
fn generate_fibonacci(num: u32){
     let num = num + 1;
    
    let mut a = 0;
    let mut b = 1;
    let mut c = 0;
    // let mut _array = [0; 50]; // 创建一个长度为50的数组，不可修改大小
    let mut _array = vec![0; num as usize]; //动态数组使用vec声明，此处是关键

    if num == 1 {
        _array[0] = a;
    }else if num == 2 {
        _array[0] = a;
        _array[1] = b;
    }else if num > 2 {
        _array[0] = a;
        _array[1] = b;

        for i in 2..num {
            c = a + b;
            a = b;
            b = c;
            _array[i as usize] = c;
        }
    }

    println!("_array={:?}", _array);
}

//打印n的斐波拉契数值
fn print_fibonacci_array(num: u32) -> u32{
    match num {
        0 => 0,
        1 => 1,
        _ => print_fibonacci_array(num - 2) + print_fibonacci_array(num - 1),
    }
}