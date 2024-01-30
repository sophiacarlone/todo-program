// // Sophia Carlone
// //Todo program

use winit::window::{self, Fullscreen, Window};

fn main() {
    let main_window = MainWindow::new().unwrap();
    // main_window.window().set_fullscreen();
    // main_window.window().set_fullscreen(true);
    // main_window.window().set_size(|winit_win: &winit::window::Window| {
    //     winit_win.set_fullscreen(Some(winit::window::Fullscreen::Borderless(winit_win.current_monitor())));
    // });
    // main_window.window().set_size(100);
    main_window.run().unwrap();
}

//slint language inside
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
        // Text{
        //     text:"kill Peter";
        // }
    }
    
    export component MainWindow inherits Window {
        width: 1800px;
        height: 1200px;
        // preferred-width: 100%;
        // preferred-height: 100%;
        // property window_width <=> width;
        // property window_height <=> height;
        background: #8374D5; //Pulled this out of my butt and it turns out to be a really pretty purple
        in property <[TaskData]> tasks: [
            {label: "kill peter", completed: false, priority: 0, due: 0},
            {label: "kill cary", completed: false, priority: 0, due: 0}
        ];
        for task[i] in tasks : Task {
            x:i*64px;
            y:i*64px;
            width: 64px;
            height: 64px;
            background: #3960D5;
            Text{
                text: task.label;
            }
        }
    }
}

// std::env::set_var("SLINT_FULLSCREEN", "1");

/*TODOs in order */
//TODO: Fullscreen //there is some kind of splint full screen thing. For now (1800 x 1200) is ok
//TODO: Basic Layout
//TODO: test wrapping
//TODO: Drag and drop
//TODO: custom colors


/***********GARBAGE SHOOT***********/

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
