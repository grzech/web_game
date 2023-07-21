use rocket::serde::{Deserialize, json::Json};
use std::{fs::OpenOptions, io::Write};

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello World!"
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct Task<'r> {
    item: &'r str,
}

#[post("/addtask", data="<task>")]
fn add_task(task: Json<Task<'_>>) -> &'static str {
    let mut tasks = OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open("tasks.txt")
        .expect("Unable to open tasks.txt");
    let task_item_string = format!("{}\n", task.item);
    let task_item_bytes = task_item_string.as_bytes();
    tasks.write(task_item_bytes).expect("Cant write to tasks.txt");
    "Task added successfully"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, add_task])
}