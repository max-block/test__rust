use std::collections::HashMap;

#[derive(Debug)]
struct Item {
    name: String,
    description: String,
    link: String,
}

fn main() {
    let mut data_map = HashMap::new();
    data_map.insert("a".to_string(), "aaa".to_string());
    data_map.insert("b".to_string(), "bbb".to_string());

    let link = "http://site.com".to_string();

    let res: Vec<_> = data_map
        .into_iter()
        .map(|(k, v)| Item {
            name: k,
            description: v,
            link: link.clone(),
        })
        .collect();

    dbg!(res);
}
