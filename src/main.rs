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

    let delete_completed = Button::new("Deleted Checked Boxes")
        .on_click(|_, data: &mut TodoList, _|{
            //TODO: go through the vector and find and delete all the ones that were completed
            // let mut v_iter = data.todos.iter();
            data.todos.retain(|x| !x.completed );
            // for i in v_iter{
            //     if i.completed{
            //         data.todos.remove(
            //     }
            // }
        });

    // Try to get this on center top
    Flex::column()
        .with_child(Flex::row()
            .with_child(TextBox::new().lens(TodoList::text))
            .with_child(add_new))
        .with_child(delete_completed)
        .with_child(day)
        .scroll()
    }

fn main() {
    let main_window = WindowDesc::new(build_ui())
        .window_size((600.0, 400.0))
        .title("My ToDos");

    AppLauncher::with_window(main_window)
        .launch(TodoList::default())
        .expect("Failed to launch application");
}
