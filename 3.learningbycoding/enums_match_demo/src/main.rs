enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),//绑定值的模式
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

fn main() {
    //绑定值的模式
    value_in_cents(Coin::Quarter(UsState::Alabama));

    // 匹配Option<T>
    let five  = Some(5);
    let six = plus_one(five);
    let _none  = plus_one(None);
    println!("six= {:?}; _none={:?}", six, _none);
}


fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn plus_one(x: Option<u32>) -> Option<u32> {
    match x {// // 会在 Rust 代码中看到很多这样的模式：match 一个枚举，绑定其中的值到一个变量，接着根据其值执行代码。这在一开始有点复杂，不过一旦习惯了，你会希望所有语言都拥有它！这一直是用户的最爱。
        None => None,//Rust 中的匹配是 穷尽的（exhaustive）：必须穷举到最后的可能性来使代码有效。删除此行，编译器会报错
        Some(i) => Some(i +1),
    }
}