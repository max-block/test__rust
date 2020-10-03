#[derive(Debug)]
struct Data {
    name: String,
    value: i32,
}

fn main() {
    let data_list = vec![
        Data {
            name: "a".to_string(),
            value: 1,
        },
        Data {
            name: "b".to_string(),
            value: 1,
        },
    ];

    let res = data_list.iter().find(|x| x.name == "b".to_string());
    dbg!(res);
}
