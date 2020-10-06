fn main() {
    // let data: Vec<Result<i32, i8>> = vec![Ok(1), Ok(2), Ok(3)];
    let data: Vec<Result<i32, i8>> = vec![Ok(1), Ok(2), Ok(3), Err(4)];
    let res: Result<Vec<i32>, i8> = data.into_iter().collect();
    println!("{:?}", res);
}
