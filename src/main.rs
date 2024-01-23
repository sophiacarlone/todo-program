// Sophia Carlone
//Todo program

//TODO: tasks need an bool (done) and optional due date (and maybe priority)

use std::io::{self, Read};
fn main() {
    println!("heyyyy we are starting! A gaboghoul");

    // let mut  = Vec::new();
    let mut tasks = Vec::new();
    tasks.push("hi");

    let mut option = String::new();
    println!("(1) Insert, (2) Delete, (3) Move");
    io::stdin()
        .read_line(&mut option).expect("input failed");

    if option == "1"{
        let mut temp = String::new();
        println!("please enter your first task");
        io::stdin()
            .read_line(&mut temp).expect("input failed"); //for the .expect, there is a Ok or an Err returned. If and Err is returned, it prints out the message after a panic
        println!("{} was inputed", temp); //how to print out
        tasks.push(&mut temp);    
    }
    else if option == "2" {

    }
    else if option == "3" {
        
    }
    else{
        println!("you've done something wrong");
    }

    println!("separation");
    for x in &tasks {
        println!("{x}");
    }

}
