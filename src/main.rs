// // Sophia Carlone
// //Todo program

fn main() {
    MainWindow::new().unwrap().run().unwrap();
}

slint::slint! {
    struct TaskData{
        label: string,
        completed: bool,
        priority: int,
        due: int, //this will be changed later as I learn more about time libraries in this language
    }

    component Task inherits Rectangle {
        width: 64px;
        height: 64px;
        background: #3960D5;
        Text{
            text:"kill Peter";
        }
    }

    export component MainWindow inherits Window {
        width: 100px;
        height: 100px;
        background: #8374D5; //Pulled this out of my butt and it turns out to be a really pretty purple
        in property <[TaskData]> tasks: [
            {label: "kill peter", completed: false, priority: 5, due: 0},
            {label: "kill cary", completed: false, priority: 5, due: 0}
        ];
        for task[i] in tasks : Task {
            x:i*64px*2;
            y:i*64px*2;
            width: 64px;
            height: 64px;
            background: #3960D5;
            Text{
                text: task.label;
            }
        }
    }
}

// //TODO: tasks need an bool (done) and optional due date (and maybe priority)

// use std::io;
// fn main() {
//     println!("heyyyy we are starting! A gaboghoul");

//     // let mut  = Vec::new();
//     let mut tasks = Vec::new();
//     tasks.push("hi");

//     let mut option = String::new();
//     println!("(1) Insert, (2) Delete, (3) Move");
//     io::stdin()
//         .read_line(&mut option).expect("input failed");

//     if option == "1"{
//         let mut temp = String::new();
//         println!("please enter your first task");
//         io::stdin()
//             .read_line(&mut temp).expect("input failed"); //for the .expect, there is a Ok or an Err returned. If and Err is returned, it prints out the message after a panic
//         println!("{} was inputed", temp); //how to print out
//         tasks.push(&mut temp);    
//     }
//     else if option == "2" {

//     }
//     else if option == "3" {
        
//     }
//     else{
//         println!("you've done something wrong");
//     }

//     println!("separation");
//     for x in &tasks {
//         println!("{x}");
//     }

// }
