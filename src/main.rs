//Sophia Carlone

use druid::{AppLauncher, Data, Lens, Widget, WidgetExt, WindowDesc, Env};
use druid::widget::{Button, Checkbox, Flex, Label,  List,  TextBox};
use im::Vector; //cannot use standard vector

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
                let new_task = data.text.clone();
                data.text = "".to_string();
                data.todos.push_back(TaskData {label: new_task, priority: 1, completed: false});
            }
    });

    Flex::column()
        .with_child(day)
        .with_child(Flex::row()
            .with_child(TextBox::new().lens(TodoList::text))
            .with_child(add_new))
}

fn main() {
    let main_window = WindowDesc::new(build_ui())
        .window_size((600.0, 400.0))
        .title("My ToDos");

    AppLauncher::with_window(main_window)
        .launch(TodoList::default())
        .expect("Failed to launch application");
}
