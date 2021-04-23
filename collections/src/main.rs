fn main() {
    let v: Vec<i32> = Vec::new();

    let initial_v = vec![2,3,4];

    let mut update_v = Vec::new();

    update_v.push(5);
    update_v.push(6);
    update_v.push(5);


    // READ vectors
    let third: i32 = &initial_v[2];

    // or...
    match v.get(2) {
        Some(third) => println!("The 3rd element is {}", third),
        None => println!("no 3rd element present"),
    }


}
