//记住：表达式的结尾没有分号，可以返回值；如果在表达式的结尾加上分号，它就变成了语句，语句不会返回值。

fn main() {
    // 1.函数
    // Rust 代码中的函数和变量名使用 snake case 规范风格。在 snake case 中，所有字母都是小写并使用下划线分隔单词。
    another_function(6);
    print_labled_measurment(8,'f');

    // 2.语句和表达式
    // 函数体由一系列的语句和一个可选的结尾表达式构成。因为 Rust 是一门基于表达式（expression-based）的语言，这是一个需要理解的（不同于其他语言）重要区别。
    // 语句（Statements）是执行一些操作但不返回值的指令。表达式（Expressions）计算并产生一个值。
    //表达式会计算出一个值，并且你将编写的大部分 Rust 代码是由表达式组成的。
    // 表达式可以是语句的一部分：语句 let y = 6; 中的 6 是一个表达式，它计算出的值是 6。函数调用是一个表达式。宏调用是一个表达式。用大括号创建的一个新的块作用域也是一个表达式

    let y =  {
        let x = 3;
        x + 1 // 注意 x+1 这一行在结尾没有分号。表达式的结尾没有分号。如果在表达式的结尾加上分号，它就变成了语句，而【语句不会返回值】。
    };
    println!("The value of y is: {}", y);

    // 3.具有返回值的函数
    // 函数可以向调用它的代码返回值。我们并不对返回值命名，但要在箭头（->）后声明它的类型。在 Rust 中，函数的返回值等同于函数体最后一个表达式的值。
    // 如果函数没有返回值，它会返回一个 `()`，表示一个空元组。
    let x = five();
    println!("The value of x is: {}", x);

    let z = plus_one(8);
    println!("The value of z is: {}", z);
}

// note: another_function 定义在 main 函数 之后；也可以定义在之前。Rust 不关心函数定义于何处，只要定义了就行
fn another_function(x: i32) {//在函数签名中，必须 声明每个参数的类型。这是 Rust 设计中一个经过慎重考虑的决定：要求在函数定义中提供类型注解，意味着编译器再也不需要你在代码的其他地方注明类型来指出你的意图
    println!("The value of x is {}", x);
}


fn print_labled_measurment(value: i32, unit_lable: char) {
    println!("The measurement is {}{}", value, unit_lable);
}


fn five() -> i32 {
    5 // 加分号；编译报错
}

fn plus_one(x: i32) -> i32 {
    x + 1 // 加分号；编译报错
}