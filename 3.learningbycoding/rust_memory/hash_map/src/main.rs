// 功能说明：
// 当 HashMap::new() 时，它并没有分配空间，容量为零
// 随着哈希表不断插入数据，它会以 2 的幂减一的方式增长，最小是 3。
// 当删除表中的数据时，原有的表大小不变，只有显式地调用 shrink_to_fit，才会让哈希表变小。

use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    explain("new", &map);

    map.insert('a',1);
    explain("added 1", &map);

    map.insert('b',2);
    map.insert('c',3);
    explain("added 3", &map);

    map.insert('d',4);
    explain("added 4", &map);

    //get 时需要引用并返回引用
    assert_eq!(map.get(&'a'),Some(&1));
    assert_eq!(map.get_key_value(&'b'), Some((&'b',&2)));

    //删除键值对a,1
    map.remove(&'a');
    assert_eq!(map.contains_key(&'a'), false);
    assert_eq!(map.get(&'a'), None);
    explain("removed", &map);

    //shrink 哈希表变小了
    map.shrink_to_fit();
    explain("shrinked", &map);
}


fn explain<K, V>(name: &str, map: &HashMap<K, V>) {
    println!("{}:len:{},cap:{}", name, map.len(),map.capacity());

    //
}