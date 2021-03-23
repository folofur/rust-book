use std::io;

fn main() {
    println!("Which temp conversion would you like?");
    println!("For celcius to farhenheit, press 1");
    println!("For farhenheit to celcius, press 2");

    let mut decide = String::new();
    io::stdin()
        .read_line(&mut decide)
        .expect("failed to read line");

    let decide: &str = decide.trim();
    println!("you chose {}", decide);
    
    if decide == "1" {
        println!("What is your celcius temperature to convert? (enter a number)");
        convert("c");
    } else if decide == "2" {
        println!("What is your farhenheit temperature to convert? (enter a number");
        convert("f");
    }

}

fn convert(t:&str, num:&str) {
    let mut temp = num;
    if t == "c" {
        let temp = num.parse(){
            Ok(num) => num,
            Err(_) => continue,
        }
        println!("{}" , num)
    }
}
