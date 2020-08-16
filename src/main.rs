#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello world"
}

#[get("/search?<cmd>")]
fn search(cmd: String) -> &'static str {
    println!("cmd: {}", cmd);
    "Hello from the search page"
}

fn main() {
    rocket::ignite().mount("/", routes![index, search]).launch();
}

fn get_cmd_from_query(query: &str) -> &str {
    let test = "hello";
    return &test;
}

