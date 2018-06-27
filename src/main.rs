#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;

extern crate rocket_contrib;
use rocket_contrib::Template;
use rocket::request::{Form};

use std::collections::HashMap;

extern crate notify_rust;
use notify_rust::Notification;

mod static_files;

#[derive(FromForm)]
struct Message {
  title: String,
  message: String
}

#[post("/", data = "<msg>")]
fn post_index(msg: Form<Message>) -> Template {
  let m = msg.get();
  Notification::new()
     .summary( &format!("{}", m.title) )                                                                                                                
     .body( &format!("{}",m.message) )
     .icon("dialog-information")
     .show().unwrap();

  let mut context = HashMap::new();
  context.insert("msg", "Message sent.");

  Template::render("index", context)

}


#[get("/")]
fn get_index() -> Template {
  let mut context = HashMap::new();
  context.insert("", "");

  Template::render("index", context)
}

fn main() {
    rocket::ignite()
      .mount("/", routes![get_index, post_index, static_files::all])
      .attach(Template::fairing())
      .launch();
}

