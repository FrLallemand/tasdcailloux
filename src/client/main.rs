extern crate tasdcailloux;
extern crate nanomsg;
extern crate gdk_pixbuf;
extern crate gtk;
extern crate bincode;
#[macro_use]extern crate lazy_static;
extern crate futures;

pub mod ui;
pub mod connection;


use ui::App;
use connection::*;
use tasdcailloux::models::element::Element;

fn main() {
    /*
    let mut socket = Socket::new(Protocol::Req).unwrap();
    socket.connect(&"tcp://127.0.0.1:5555").expect("Fail to bind to tcp port");

    let message = Message{ message_type: MessageType::GetCount };
    let encoded: Vec<u8> = serialize(&message, Infinite).unwrap();
    socket.write(&encoded).unwrap();
    let mut msg = Vec::new();
    socket.read_to_end(&mut msg).unwrap();
    let decoded: Result<i32, Error> = deserialize(&msg).unwrap();
    match decoded{
        Ok(element) => println!("{:?}", element),
        Err(e) => println!("Error : {:?}",e)
    }
     */
    //let mut socket = establish_connection();
    let origin_list_result = get_origin_list();
    let origin_list: Vec<Element> = match origin_list_result {
        Ok(elements) => {
            elements
        },
        Err(_) => {
            Vec::new()
        }
    };

    App::new(origin_list)
        .connect_events()
        .then_execute();

}
