//Sophia Carlone

use druid::{AppLauncher, Color, Data, Lens, Widget, WidgetExt, WindowDesc, Env};
use druid::widget::{Button, Checkbox, Container, Flex, Label, LensWrap, List, Split};
use im::Vector;
use task_data_derived_lenses::label; //cannot use standard vector


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
}


/*****FUNCTIONS*****/
fn build_ui() -> impl Widget<TodoList>{ //root widget
    let day = List::new( || {
        Flex::row()
            .with_child(Checkbox::new("").lens(TaskData::completed))
            .with_child(Label::new(|data: &TaskData, _: &Env| data.label.clone()))
    }).lens(TodoList::todos);

    Flex::column()
        .with_child(day)
        .with_child(Button::new("add task")
            .on_click(|_, data: &mut TodoList, _|{
                data.todos.push_back(TaskData {label: String::from("hi"), priority: 1, completed: false});
            }))
    // Split::columns(
    //     Container::new(
    //         LensWrap::new(
    //             List::new(|| Label::dynamic(|data, _| format!("List item: {data}"))),
    //             TodoList::todos::label,
    //         )

    //     )
    // .border(Color::grey(0.6), 2.0),
    // Container::new(
    //     Flex::column()
    //     .with_flex_child(Button::new("add item"), 1.0)
    //     .with_flex_child(Label::new("Textbox placeholder"), 1.0),
    // )
    // .border(Color::grey(0.6), 2.0),)
    // .draggable(true)
}

fn main() {
    let main_window = WindowDesc::new(build_ui())
        .window_size((600.0, 400.0))
        .title("My ToDos");

    // let mut test = TaskData{
    //     label: String::from("jo"),
    //     priority: 1,
    //     completed: false,
    // };
    // let testvec = Vector<TaskData>;

    // let mut initial_data = TodoList{
    //     todos: 
    // };

    AppLauncher::with_window(main_window)
        .launch(TodoList::default())
        .expect("Failed to launch application");
}
