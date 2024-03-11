//Sophia Carlone

// use druid::platform_menus::mac::file::print;
use druid::{AppLauncher, Data, Lens, Widget, WidgetExt, WindowDesc, Env};
use druid::widget::{Button, Checkbox, Flex, Label,  List,  TextBox};
use im::Vector; //cannot use standard vector
use std::fs::File;
use std::fs;
use std::path::Path;
use std::io::{self, BufRead, Write};
use std::fs::OpenOptions;
// use std::fs::read_to_string;


/******DATA*****/
#[derive(Clone, Data, Lens, Default)]
struct TaskData {
    label: String,
    priority: i32,   
    completed: bool, 
}

#[derive(Clone, Data, Lens, Default)]
struct TodoList{
    todos: Vector<TaskData>,
    text: String,
}


/*****FUNCTIONS*****/
fn build_ui() -> impl Widget<TodoList>{ //root widget

    let day = List::new( || {
        Flex::row()
            .with_child(Checkbox::new("").lens(TaskData::completed))
            .with_child(Label::new(|data: &TaskData, _: &Env| data.label.clone()))
    }).lens(TodoList::todos);

    let add_new = Button::new("add task")
        .on_click(|_, data: &mut TodoList, _|{
            if  data.text.trim() != ""{
                let mut file = OpenOptions::new()
                    .append(true)
                    .open("./src/tasks.txt")
                    .expect("failed to open file"); //TODO find better place for it
                let new_task = data.text.clone();
                data.text = "".to_string();
                file.write(new_task.as_bytes()).expect("write failed");
                data.todos.push_back(TaskData {label: new_task, priority: 1, completed: false});
            }
    });

    let delete_completed = Button::new("Delete Completed Tasks")
        .on_click(|_, data: &mut TodoList, _|{
            data.todos.retain(|x| !x.completed );
            fs::remove_file("./src/tasks.txt").expect("could not remove file");
            let mut file = File::create("./src/tasks.txt").expect("failed to open file"); //TODO find better place for it
            for t in data.todos.iter(){
                file.write(t.label.as_bytes()).expect("write failed");
            }
        });

    //TODO: Try to get this on center top
    Flex::column()
        .with_child(Flex::row()
            .with_child(TextBox::new().lens(TodoList::text))
            .with_child(add_new))
        .with_child(delete_completed)
        .with_child(day)
        .scroll()
    }

fn main() {
    let mut start_vec = Vector::<TaskData>::new();

    if let Ok(lines) = read_lines("./src/tasks.txt"){
        for line in lines.flatten(){ //turns iterator to the original type
            let t = TaskData{
                label: line, 
                priority: 1, 
                completed: false
            };
            start_vec.push_back(t);
        }
    }

    let start = TodoList{
        todos: start_vec,
        ..Default::default()
    };
    
    let main_window = WindowDesc::new(build_ui())
        .window_size((600.0, 400.0))
        .title("My ToDos");

    AppLauncher::with_window(main_window)
        .launch(start)
        .expect("Failed to launch application");
}

//taken from rust book
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines()) //iterator over reader
}
