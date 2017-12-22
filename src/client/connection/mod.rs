use std::io::{Read, Write};
use nanomsg::{Socket, Protocol};
use bincode::{serialize, deserialize, Infinite};
use tasdcailloux::models::{Message, Error};
use tasdcailloux::models::MessageType as AppMessageType;
use tasdcailloux::models::element::Element;

pub fn establish_connection() -> Socket {
    let mut socket = Socket::new(Protocol::Req).unwrap();
    socket.connect(&"tcp://127.0.0.1:5555").expect("Fail to bind to tcp port");
    //socket.set_linger(-1).expect("cannot set linger");
    socket.set_send_timeout(5).unwrap();
    socket.set_receive_timeout(5).unwrap();
    socket
}

pub fn get_origin_list(mut socket: Socket) -> Result<Vec<Element>, Error>{
    let message = Message{ message_type: AppMessageType::GetAll };
    let encoded: Vec<u8> = serialize(&message, Infinite).unwrap();

    match socket.write(&encoded) {
        Ok(_) => {
        },
        Err(_) => {
        }
    };

    let mut msg = Vec::new();
    match socket.read_to_end(&mut msg) {
        Ok(_) => {
            deserialize(&msg).unwrap()
        },
        Err(_) => {
            Err(Error::InternalError)
        }
    }
}
/*
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
