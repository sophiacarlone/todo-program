//Sophia Carlone

use druid::{AppLauncher, Widget, WindowDesc, Color};
use druid::widget::{Container, Flex, Split, Label, List};
use im::Vector; //cannot use standard vector

/******DATA*****/
struct Task_Data {
    label: String,
    priority: i32,    
}
type TodoList = Vector<Task_Data>;


/*****FUNCTIONS*****/
fn build_ui() -> impl Widget<i32>{ //root widget
    Split::columns(
        Container::new(
        Flex::column()
            .with_flex_child(Label::new("first item"), 1.0)
            .with_flex_child(Label::new("second item"), 1.0)
            .with_flex_child(Label::new("third item"), 1.0)
            .with_flex_child(Label::new("fourth item"), 1.0),
    )
    .border(Color::grey(0.6), 2.0),
    Container::new(
        Flex::column()
            .with_flex_child(Label::new("Button placeholder"), 1.0)
            .with_flex_child(Label::new("Textbox placeholder"), 1.0),
    )
    .border(Color::grey(0.6), 2.0),)
    .draggable(true)
}

fn main() {
    let main_window = WindowDesc::new(build_ui())
        .window_size((600.0, 400.0))
        .title("My ToDos");
    let x = 5;
    let initial_data = x;

    AppLauncher::with_window(main_window)
        .launch(initial_data)
        .expect("Failed to launch application");
}
