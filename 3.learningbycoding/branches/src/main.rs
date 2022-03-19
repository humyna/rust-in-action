//https://kaisery.github.io/trpl-zh-cn/ch03-05-control-flow.html

fn main() {
    // 1.if表达式
    let number  = 3;

    if number < 6 { // number < 6 不要用括号
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // if number { // number 不能自动转换为 bool
    //     println!("number was three");
    // } else {
    //     println!("number was something else");
    // }

    if number != 0 {
        println!("number was something other than zero");
    }

    // 多重处理
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else {
        println!("number is not divisible by 4 or 3");
    }

    // if let
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };//记住，代码块的值是其最后一个表达式的值，而数字本身就是一个表达式。

    println!("The value of number is: {}", number);

    // 2. 循环：loop、while 和 for
    // 2.1 loop 关键字告诉 Rust 一遍又一遍地执行一段代码直到你明确要求停止
    // loop {
    //     println!("again!");
    // }

    //循环标签 ：如果存在嵌套循环，break 和 continue 应用于此时最内层的循环。你可以选择在一个循环上指定一个 循环标签（loop label），然后将标签与 break 或 continue 一起使用，使这些关键字应用于已标记的循环而不是最内层的循环。
    let mut count = 0;
    'counting_up: loop { // 注意循环标签使用非闭单引号 'counting_up 表示，而不是闭合的 'counting_up'
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 5 {
                break;
            }

            if count ==2 {
                break 'counting_up;
            }

            remaining -= 1;
        }

        count += 1;
    }
    
    println!("End count = {}", count);

    // 从循环返回值
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);

    // 2.2 while ,这种结构消除了很多使用 loop、if、else 和 break 时所必须的嵌套，这样更加清晰。
    let mut number = 6;

    while number != 0 {
        println!("number = {}", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");

    // 2.3 for
    // for 循环的安全性和简洁性使得它成为 Rust 中使用最多的循环结构。
    // 即使是在想要循环执行代码特定次数时，大部分 Rustacean 也会使用 for 循环。
    // 这么做的方式是使用 Range，它是标准库提供的类型，用来生成从一个数字开始到另一个数字之前结束的所有数字的序列。
    let arr: [i32; 5] = [3, 4,6,7,1]; //注意数组初始化的写法[i32; 5] 中是分号，而不是逗号。
    let mut index = 0;
    while index < 5 {
        println!("arr[{}] = {}", index, arr[index]);
        index += 1;
    }

    // index = 0; //注意：注释这行，下面的代码仍不会报错
    for element in arr.iter() { // 或者 for element in arr {}，这种写法增强了代码安全性，并消除了可能由于超出数组的结尾或遍历长度不够而缺少一些元素而导致的 bug。
        println!("arr[{}] = {}", index, element);
        index += 1;
    }

    // 使用 for 循环来倒计时
    for number in (1..4).rev() {//rev反转 range，这样就可以从大到小遍历
        println!("{}", number);
    }
    println!("LIFTOFF!!!");
}
