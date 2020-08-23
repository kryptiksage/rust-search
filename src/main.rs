#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::response::Redirect;
mod utils;

#[get("/")]
fn index() -> &'static str {
    "Hello world"
}

#[get("/search?<cmd>")]
fn search(cmd: String) -> Redirect {
    println!("cmd: {}", cmd);
    let command = utils::get_cmd_from_query(&cmd);
    let redirect_url = match command {
        "tw" => utils::twitter::construct_twitter_url(&cmd),
        "gl" => utils::gitlab::construct_gitlab_url(&cmd),
        _ => utils::google::construct_google_search_url(&cmd)
    };        
    Redirect::to(redirect_url)
}

fn main() {
    rocket::ignite().mount("/", routes![index, search]).launch();
}
