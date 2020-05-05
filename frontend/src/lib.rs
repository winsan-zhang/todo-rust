#[macro_use]
extern crate seed;

use seed::{*,prelude::*};
use futures::Future;
use mytodo::{ JsonApiResponse, Task };


#[derive(Clone)]
enum Direction {
    Coming,
    Going,
}

struct Model {
    tasks: Vec<Task>,
}

impl Default for Model {
   fn default() -> Self {
       Self {
           tasks: vec![]
       }
   }
}



#[derive(Clone)]
enum Msg {
    FetchTasks,
    FetchedTasks(fetch::ResponseDataResult<JsonApiResponse>),
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::FetchTasks => {
            orders.perform_cmd(fetch_drills());
            orders.skip();
        },
        Msg::FetchedTasks(Ok(mut result)) => {
            model.tasks.clear();
            model.tasks.append(&mut result.data)
        },
        Msg::FetchedTasks(Err(reason)) => {
            log!(format!("Error fetching: {:?}", reason));
            orders.skip();
        },
    }
}

fn view(model: &Model) -> impl View<Msg> {
    let tasks: Vec::<Node<Msg>> = model.tasks.iter().map(|t| {
        li![{ t.title.clone() }]
    }).collect();
    h1![
        {"task"},
        ul![
            tasks
        ],
    ]
}

fn fetch_drills() -> impl Future<Output = Result<Msg, Msg>> {
    Request::new("http://localhost:8000/tasks/").fetch_json_data(Msg::FetchedTasks)
}

fn after_mount(_: Url, orders: &mut impl Orders<Msg>) -> AfterMount<Model> {
    orders.send_msg(Msg::FetchTasks);
    AfterMount::default()
}

#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view).after_mount(after_mount).build_and_start();
}
