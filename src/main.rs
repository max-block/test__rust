fn main() {
    let data = vec!["a".to_string(); 10];
    let res:Vec<Vec<String>> = data.chunks(3).map(|x|x.to_vec()).collect();
    println!("{:?}", res);
}
