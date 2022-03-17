//https://kaisery.github.io/trpl-zh-cn/ch03-02-data-types.html

fn main() {
    // 1.整型
    // 类型注解u32，去掉会报错
    let _guess : u32 =  "39".parse().expect("Not a number!");
    
    // 不同进制
    // 多种数字类型的数字字面值允许使用类型后缀，例如 57u8 来指定类型，同时也允许使用 _ 做为分隔符以方便读数，例如1_000，它的值与你指定的 1000 相同
    let _data_d: u32 = 101_000;
    let _data_h: u32 = 0x10e;
    let _data_o: u32 = 0o107;
    let _data_b: u32 = 0b111_000;
    let _data_byte: u8 = b'B';
    println!("_data_d={},_data_h={},_data_o={},_data_b={},_data_byte={}",_data_d, _data_h, _data_o,_data_b,_data_byte);

    // 整型溢出
    // 在 debug 模式编译时，Rust 检查这类问题并使程序 panic
    // 在 release 构建中，Rust 不检测溢出，相反会进行一种被称为二进制补码包装（two’s complement wrapping）的操作。简而言之，值 256 变成 0，值 257 变成 1。
    // 标准库中有一个类型显式提供此功能，Wrapping TODO 
    //let _idata: i8 = 128;
    //let _udata: u8 = 256;

    // 2.浮点
    // Rust 的浮点数类型是 f32 和 f64，分别占 32 位和 64 位。
    // 默认类型是 f64，因为在现代 CPU 中，它与 f32 速度几乎一样，不过精度更高
    // 所有的浮点型都是有符号的。
    // 浮点数采用 IEEE-754 标准表示
    let _data_f = 2.0;
    let _data_f32:f32 = 3.0;
    println!("_data_f={}, _data_f32={}", _data_f, _data_f32);

    // 3.数值运算
    let sum = 5 + 6;
    let diff = 10.24 - 2.4;
    let pro = 3 * 12;
    let quotient = 7.2 / 3.1;
    let floored = 3 / 4; //结果为0
    let remainder = 11 % 3;
    println!("sum={}, diff={}, pro={}, quotient={}, floored={}, remainder={}", sum, diff, pro, quotient, floored, remainder);

    // 4.布尔型
    let t = true;
    let f : bool = false;
    println!("t={}, f={}", t, f);

    // 5.字符类型
    // Rust的 char 类型是语言中最原生的字母类型。
    // 我们用单引号声明 char 字面量，使用双引号声明字符串字面量.
    // Rust 的 char 类型的大小为四个字节(four bytes)，并代表了一个 Unicode 标量值（Unicode Scalar Value）。
    // 在 Rust 中，拼音字母（Accented letters），中文、日文、韩文等字符，emoji（绘文字）以及零长度的空白字符都是有效的 char 值。
    let c = 'f';
    let d = '😁';
    println!("c={}, d={}", c, d);

    // 6.复合类型-元组tuple
    // 6.1 元组是一个将多个其他类型的值组合进一个复合类型的主要方式。
    // 元组长度固定：一旦声明，其长度不会增大或缩小。
    // 6.2 我们使用包含在圆括号中的逗号分隔的值列表来创建一个元组。元组中的每一个位置都有一个类型，而且这些不同值的类型也不必是相同的。
    // 为了从元组中获取单个值，可以使用模式匹配（pattern matching）来解构（destructure）元组值
    // 6.3 没有任何值的元组 () 是一种特殊的类型，只有一个值，也写成 () 。该类型被称为 单元类型（unit type），而该值被称为 单元值（unit value）。如果表达式不返回任何其他值，则会隐式返回单元值。
    let tup : (i32, f32, bool) = (341, 32.1, true);
    let (x, y, z) = tup; //解构（destructuring）
    println!("x={}, y={}, z={}, x={}", x, y, z, tup.0);//使用点号（.）后跟值的索引来直接访问


    // 7.复合类型-数组array
    // 与元组不同，数组中的每个元素的类型必须相同。
    // Rust中的数组长度是固定的
    // 当你想要在栈（stack）而不是在堆（heap）上为数据分配空间 或者是想要确保总是有固定数量的元素时 ，可以使用数组。
    // 数组并不如 vector 类型灵活。vector 类型是标准库提供的一个 允许 增长和缩小长度的类似数组的集合类型。当不确定是应该使用数组还是 vector 的时候，那么很可能应该使用 vector。
    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    println!("months={:?}", months);

    // 
    let _array: [i32; 5] = [1, 2, 3, 4, 5];//初始化数组。数组类型写法：在方括号中包含每个元素的类型，后跟分号，再后跟数组元素的数量。
    println!("_array={:?}", _array);
    let _array = [3; 5];//初始化数组。在方括号中指定初始值加分号再加元素个数的方式来创建一个每个元素都为相同值的数组
    println!("_array={:?}", _array);
    println!("_array0={}", _array[0]);//访问数组元素
    //println!("_array_e={}", _array[7]);//数组越界，编译报错
}
