use std::collections::BTreeMap;

fn main() {
    let mut map = BTreeMap::new();
    map.insert(1, "a".to_string());
    map.insert(3, "c".to_string());
    map.insert(2, "b".to_string());

    for (k, v) in &map {
        println!("{} -> {}", k, v);
    }
// 1 -> a
// 2 -> b
// 3 -> c
}
