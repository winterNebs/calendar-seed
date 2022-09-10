// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]

use std::usize;

use chrono::{DateTime, Local};
use seed::{prelude::*, *};
use ulid::Ulid;

// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app started.
fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    let todo = vec![
        TodoEntry {
            id: Ulid::new(),
            name: "make todo list".to_owned(),
            details: "details1".to_owned(),
            date: Local::now(),
            status: EntryStatus::Todo,
            children: Vec::new(),
            category: "todo".to_owned(),
        },
        TodoEntry {
            id: Ulid::new(),
            name: "make calendar".to_owned(),
            details: "details2".to_owned(),
            date: Local::now(),
            status: EntryStatus::Todo,
            children: Vec::new(),
            category: "todo".to_owned(),
        },
    ];
    Model {
        todos: todo,
        modal_active: true,
    }
}

// ------ ------
//     Model
// ------ ------

// `Model` describes our app state.
struct Model {
    todos: Vec<TodoEntry>,
    modal_active: bool,
}

// Entry, could be an event or todolist item
// If end is not set, then treat start as a "date" (ignore time)
struct TodoEntry {
    id: Ulid,
    name: String,
    details: String,
    date: DateTime<Local>,
    status: EntryStatus,
    children: Vec<Ulid>,
    category: String,
}

enum EntryStatus {
    InProgress,
    Todo,
    Done,
    Cancelled,
    OnHold,
}
// ------ ------
//    Update
// ------ ------

// (Remove the line below once any of your `Msg` variants doesn't implement `Copy`.)
#[derive(Copy, Clone)]
// `Msg` describes the different events you can modify state with.
enum Msg {
    CreateTodo,
    ToggleModal(bool),
}

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::CreateTodo => {}
        Msg::ToggleModal(active) => model.modal_active = active,
    }
}

// ------ ------
//     View
// ------ ------

// `view` describes what to display.
fn view(model: &Model) -> Node<Msg> {
    div![
        "Home page",
        view_todo_modal(model.modal_active),
        view_todo_list(&model.todos)
    ]
}

fn view_todo_modal(visible: bool) -> Node<Msg> {
    div![
        C!["modal", IF!(visible => "is-active")],
        div![C!["modal-background"]],
        div![
            C!["modal-content"],
            div![
                C!["card"],
                header![
                    C!["card-header"],
                    p![
                        C!["card-header-title"],
                        input![C!["input"], attrs! {At::Placeholder => "Title"}],
                    ]
                ],
                div![
                    C!["card-content"],
                    div![
                        C!["content"],
                        textarea![C!["textarea"], attrs! {At::Placeholder => "Description"}]
                    ]
                ],
                footer![
                    C!["card-footer"],
                    a![C!["card-footer-item"], "Save"],
                    a![
                        C!["card-footer-item"],
                        ev(Ev::Click, |_| Msg::ToggleModal(false)),
                        "Cancel"
                    ]
                ]
            ],
        ],
        button![C!["modal-close is-large"], attrs! {At::AriaLabel=>"close"}]
    ]
}

fn view_todo_list(todos: &Vec<TodoEntry>) -> Node<Msg> {
    div![
        C!["section"],
        todos.iter().map(|todo| {
            div![
                C!["card"],
                header![C!["card-header"], p![C!["card-header-title"], &todo.name]],
                div![C!["card-content"], div![C!["content"], &todo.details]]
            ]
        })
    ]
}
// ------ ------
//     Start
// ------ ------

// (This function is invoked by `init` function in `index.html`.)
#[wasm_bindgen(start)]
pub fn start() {
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view);
}
