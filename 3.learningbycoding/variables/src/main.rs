fn main() {
    //cannot assign twice to immutable variable
//    let  x = 5;
//    println!("The value of x is {}", x);
//    x = 6;
//    println!("The value of x is {}", x);

// use mut
//    let mut x = 5;
//    println!("The value of x is {}", x);
//    x = 6;
//    println!("The value of x is {}", x);

   // 常量不光默认不能变，它总是不能变。
   // 声明常量使用 const 关键字而不是 let，并且 必须 注明值的类型
   // Rust 对常量的命名约定是在单词之间使用全大写加下划线
//    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

   //隐藏 shadowing：可以定义一个与之前变量同名的新变量
   let x = 5;
   let x = x + 1;

   {
       let x = x * 2;
       println!("The value of x in the  inner scope is {}", x);
   }
   println!("The value of x is {}", x);

    //隐藏与mut的区别
    //隐藏  
   let spaces = "     ";
   let spaces = spaces.len();

   //mut error,不能改变变量的类型
//    let mut spaces = "     ";
//     spaces = spaces.len();

}
