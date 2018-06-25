extern crate notify_rust;
use notify_rust::Notification;
fn main() {
	Notification::new()
    .summary("Hello, world!")
		.body("Teste de notificação")
		.icon("dialog-information")
		.show().unwrap();
}
