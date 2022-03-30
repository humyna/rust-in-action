// 打印结构体详细信息
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let width = 30;
    let height = 40;
    println!("The area of the rectangle is {} square pixels.", area(width, height));

    //元组
    let rect = (20,30);
    println!("The area of the rectangle is {} square pixels.", area1(rect));

    // 结构体
    let rectangle = Rectangle {
        width: 40,
        height: 50,
    };
    println!("The area of the rectangle {:?} is {} square pixels.", rectangle,area2(&rectangle));
    //为了使结构体更易读，可以使用 {:#?} 替换 println! 字符串中的 {:?}
    println!("The area of the rectangle {:#?} is {} square pixels.", rectangle,area2(&rectangle));

    // 使用宏dbg!打印结构体中的某个字段
    let scale = 2;
    let rectangle2 = Rectangle {
        width: dbg!( 40 * scale),
        height: 50,
    };

    dbg!(&rectangle2);

}

// M1.计算面积
fn area(width: u32, height: u32) -> u32 {
    width * height
}

// M2:使用元组重构area()
// 两个参数是相关联的,使用元组更合理
// 由于元组并没有给出元素的名称，不易读
fn area1(dimensions:(u32,u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// M3:使用结构体重构：赋予更多意义
fn area2(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}