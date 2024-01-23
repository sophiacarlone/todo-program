// Sophia Carlone
//Todo program

use std::io;
fn main() {
    println!("heyyyy we are starting! A gaboghoul");

    // let mut  = Vec::new();
    let mut tasks = Vec::new();

    tasks.push("hi");

    println!("please enter your first task");
    let mut temp = String::new();
    io::stdin()
        .read_line(&mut temp).expect("input failed"); //for the .expect, there is a Ok or an Err returned. If and Err is returned, it prints out the message after a panic
    println!("{} was inputed", temp); //how to print out
    tasks.push(&mut temp);

    println!("separation");
    for x in &tasks {
        println!("{x}");
    }
}
