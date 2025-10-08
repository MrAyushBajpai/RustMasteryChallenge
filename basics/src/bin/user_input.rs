use std::io;

fn main(){
    let mut variable = String::new();
    println!("Enter some value: ");
    io::stdin().read_line(&mut variable).expect("Failed to read line");
    println!("{variable}");
}