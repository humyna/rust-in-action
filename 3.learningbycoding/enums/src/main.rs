#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

// 将 IP 地址的数据和 IpAddrKind 成员存储在一个 struct 中
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

// 用枚举替代结构体的优势：更简洁
enum IpAddrKindSimple {
    //V4(String),
    V4(u8, u8, u8, u8),// 用枚举替代结构体另一个优势：每个成员可以处理不同类型和数量的数据。
    V6(String),
}

//定义一个包含多个类型的枚举类
#[derive(Debug)]
enum Message {
    Qiut, //没有关联任何数据
    Move {x:i32,y: i32},//类似结构体包含命名字段
    Write(String),
    ChangeColor(i32, i32, i32),
}

// 定义方法
impl Message {
    fn call(&self){
        println!("{:?}", &self);
    }
}

fn main() {
   let FOUR = IpAddrKind::V4;//注意枚举的成员位于其标识符的命名空间中，并使用两个冒号分开。
   let SIX = IpAddrKind::V6;
   println!("{:?}", FOUR);

   let home  = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    // 直接将数据附加到枚举的每个成员上，这样就不需要一个额外的结构体了
    // 每一个我们定义的枚举成员的名字也变成了一个构建枚举的实例的函数。IpAddr::V4() 是一个获取 String 参数并返回 IpAddr 类型实例的函数调用。作为定义枚举的结果，这些构造函数会自动被定义。
    //let _home = IpAddrKindSimple::V4(String::from("127.0.0.1"));
    let _home = IpAddrKindSimple::V4(127, 0, 0, 1);
    let _loopback = IpAddrKindSimple::V6(String::from("::1"));

    // 枚举方法调用
    let _method = Message::Write(String::from("hello"));
    _method.call();

}
