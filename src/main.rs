#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

extern crate notify_rust;
use notify_rust::Notification;

#[get("/")]
fn index() -> &'static str {
  Notification::new()
     .summary("Hello, world!")                                                                                                                
     .body("Teste de notificação")
     .icon("dialog-information")
     .show().unwrap();

  "Hello, world!"
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}

