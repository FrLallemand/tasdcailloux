extern crate tasdcailloux;
extern crate nanomsg;
extern crate gdk_pixbuf;
extern crate gtk;
extern crate bincode;
#[macro_use]extern crate lazy_static;
extern crate futures;
extern crate chrono;
extern crate serde;
#[macro_use]
extern crate serde_derive;


pub mod ui;
pub mod connection;


use ui::App;

fn main() {
    App::new()
        .connect_events()
        .then_execute();

}
