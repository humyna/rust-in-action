//注意：
// 可以使结构体存储被其他对象拥有的数据的引用，不过这么做的话需要用上 生命周期（lifetimes） username:&str
// 生命周期确保结构体引用的数据有效性跟结构体本身保持一致。如果你尝试在结构体中存储一个引用而不指定生命周期将是无效的
// 详见： https://kaisery.github.io/trpl-zh-cn/ch05-01-defining-structs.html#%E7%BB%93%E6%9E%84%E4%BD%93%E6%95%B0%E6%8D%AE%E7%9A%84%E6%89%80%E6%9C%89%E6%9D%83

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// 元组结构体
// 使用没有命名字段的元组结构体来创建不同的类型
// 元组结构体实例类似于元组：可以将其解构为单独的部分，也可以使用 . 后跟索引来访问单独的值
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// 没有任何字段的类单元结构体(unit-like structs)
// 类单元结构体常常在你想要在某个类型上实现 trait 但不需要在类型中存储数据的时候发挥作用。
struct AlwaysEqual;

fn main() {
    // 1.初始化实例
    let mut _user = User {
        username: String::from("someone"),
        email: String::from("test@gmail.com"),
        active: true,
        sign_in_count: 0,
    };
    _user.email = String::from("test1@gmail.com");//注意整个实例必须是可变的；Rust 并不允许只将某个字段标记为可变。

    println!("_user username:{},email:{},active:{}, sign_in_count:{}", _user.username, _user.email, _user.active, _user.sign_in_count);

    _user = build_user(String::from("test2@gmail.com"), String::from("test2"));
    println!("_user  username:{},email:{},active:{}, sign_in_count:{}", _user.username, _user.email, _user.active, _user.sign_in_count);

    // 4.使用结构体更新语法从其他实例创建实例
    let _user2 = User {
        email: _user.email.clone(),//_user有效，可以继续使用
        username: String::from("test3"),
        .._user // ..user1 必须放在最后，以指定其余的字段应从 user1 的相应字段中获取其值; 【注意】此处如果username也是从_user1获取，则_user不可使用
    };
    println!("_user after clone  username:{},email:{},active:{}, sign_in_count:{}", _user.username, _user.email, _user.active, _user.sign_in_count);
    println!("_user2  username:{},email:{},active:{}, sign_in_count:{}", _user2.username, _user2.email, _user2.active, _user2.sign_in_count);

    // 5.元组结构体实例
    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);

    // 6.类单元结构体实例
    let _subject = AlwaysEqual;
}

// 2.通过函数初始化实例
// 同其他任何表达式一样，我们可以在函数体的最后一个表达式中构造一个结构体的新实例，来隐式地返回这个实例
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

// 3.使用字段初始化简写语法（field init shorthand）初始化实例
// 参数名与字段名都完全相同，可以省略参数名，但是不能省略字段名。
fn build_user1(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

 