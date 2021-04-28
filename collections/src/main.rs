fn main() {
    // let v: Vec<i32> = Vec::new();

    let initial_v = vec![2, 3, 4];

    let mut update_v: Vec<i32> = Vec::new();

    update_v.push(5i32);
    update_v.push(6i32);
    update_v.push(5i32);

    // READ vectors
    let third: &i32 = &initial_v[2];
    println!("the 3rd element is {}", third);

    // or...
    // this way is advantageous because it will not cause the program to crash
    // because of a reference to some non-existent item. 
    match initial_v.get(2) {
        Some(third) => println!("The 3rd element is {}", third),
        None => println!("no 3rd element present"),
    }

    //new String can initialize just like new vec.
    // a String is actually a wrapper around a vector. 
    let mut s = String::new();

    // create a String from a string literal
    // you can do this with any data with "Display" trait
    let data = "some characters";
    let mut alt_s = data.to_string();
    // or
    let alter_s = "characters to Stringify".to_string();
    alt_s.push_str("asdf");

    

}
