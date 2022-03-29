// 功能说明：
// 1.HashMap基础方法使用
// 当 HashMap::new() 时，它并没有分配空间，容量为零
// 随着哈希表不断插入数据，它会以 2 的幂减一的方式增长，最小是 3。
// 当删除表中的数据时，原有的表大小不变，只有显式地调用 shrink_to_fit，才会让哈希表变小。

// 2.打印HashMap内存
// 可以使用std::mem::transmute 方法，来把数据结构打出来
// HashMap 结构有两个 u64 的RandomState，然后是4个usize
// 分别是bucket_size, ctrl, growth_left and items
// 我们 transmute 打印之后，再transmute回去，就可以看到哈希表的内存结构
use std::collections::HashMap;

fn main() {
    // 1.HashMap基本功能
    // let mut map = HashMap::new();
    // explain("new", &map);

    // map.insert('a',1);
    // explain("added 1", &map);

    // map.insert('b',2);
    // map.insert('c',3);
    // explain("added 3", &map);

    // map.insert('d',4);
    // explain("added 4", &map);

    // //get 时需要引用并返回引用
    // assert_eq!(map.get(&'a'),Some(&1));
    // assert_eq!(map.get_key_value(&'b'), Some((&'b',&2)));

    // //删除键值对a,1
    // map.remove(&'a');
    // assert_eq!(map.contains_key(&'a'), false);
    // assert_eq!(map.get(&'a'), None);
    // explain("removed", &map);

    // //shrink 哈希表变小了
    // map.shrink_to_fit();
    // explain("shrinked", &map);

    // 2.打印HashMap内存结构
    let map1 = HashMap::new();
    let mut map1 = explain_mem("new", map1);//不插入数据，此处编译会报错，因为map1是空的
    map1.insert('a', 1) ;
    let mut map1 = explain_mem("added 1", map1);

    map1.insert('b',2);
    map1.insert('c',3);
    let mut map1 = explain_mem("added 3", map1);

    map1.insert('d',4);
    let mut map1 = explain_mem("added 4", map1);

    map1.remove(&'a');
    explain_mem("final", map1);
}


fn explain<K, V>(name: &str, map: &HashMap<K, V>) {
    println!("{}:len:{},cap:{}", name, map.len(),map.capacity());
}

// 为了打印内存：可以使用std::mem::transmute 方法，来把数据结构打出来
// HashMap 结构有两个 u64 的RandomState，然后是4个usize
// 分别是bucket_size, ctrl, growth_left and items
// 我们 transmute 打印之后，再transmute回去，就可以看到哈希表的内存结构
fn explain_mem<K, V>(name: &str, map: HashMap<K, V>) -> HashMap<K, V>{
    // println!("{}:len:{},cap:{}", name, map.len(),map.capacity());
    let arr: [usize; 6] = unsafe { std::mem::transmute(map)} ;
    println!("{}: bucket_mask ox{:x}, ctrl ox{:x}, growth_left:{}, items:{}", name, arr[2],arr[3], arr[4], arr[5]);
    unsafe { std::mem:: transmute(arr) }
}