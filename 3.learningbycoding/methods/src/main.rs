// 方法（method）与函数的相同点和区别
// 方法（method）与函数类似：它们使用 fn 关键字和名称声明，可以拥有参数和返回值，同时包含在某处调用该方法时会执行的代码。
// 不过方法与函数是不同的，因为它们在结构体的上下文中被定义（或者是枚举或 trait 对象的上下文），并且它们第一个参数总是 self，它代表调用该方法的结构体实例。
// https://kaisery.github.io/trpl-zh-cn/ch05-03-method-syntax.html


#[derive(Debug)]
struct Rectangle {
    width: u32, 
    height: u32,
}

// 每个 impl 块都可以包含多个方法，每个方法都可以有不同的参数和返回值。
impl Rectangle {
    //它们第一个参数总是 self，它代表调用该方法的结构体实例
    // 选择 &self 的理由：我们并不想获取所有权，只希望能够读取结构体中的数据，而不是写入。如果想要在方法中改变调用方法的实例，需要将第一个参数改为 &mut self
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // getter 方法的实现技巧：将方法的名称与结构中的一个字段相同，与字段同名的方法将被定义为只返回字段中的值，而不做其他事情
    fn width(&self) -> bool {
        self.width > 0
    }

    // 关联函数（associated function）
    // 定义不以 self 为第一参数的关联函数（因此不是方法），因为它们并不作用于一个结构体的实例
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

// 每个结构体都允许拥有多个 impl 块。
// 此处方法没有必要单独放到一个impl中，仅用于说明语法。
impl Rectangle {
    // 多个参数的方法
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
}
}


fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 40,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 20,
    };
    
    let rect3 = Rectangle {
        width: 40,
        height: 50,
    };

    println!("The area of the rectangle is {} square pixels.", rect1.area());

    if rect1.width() {
        println!("The width of the rectangle is {}", rect1.width);
    }

    // 多个参数的方法
    println!("can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // 关联函数
    // 使用结构体名和 :: 语法来调用这关联函数
    println!("The area of the square is {} square pixels.", Rectangle::square(10).area());
}
