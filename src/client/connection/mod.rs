use std::io::{Read, Write};
use std::sync::Mutex;

use nanomsg::{Socket, Protocol,  PollFd, PollRequest, PollInOut};
use bincode::{serialize, deserialize, Infinite};
use tasdcailloux::models::{Message, Error, Response, ResponseType, ListCache};
use tasdcailloux::models::MessageType as AppMessageType;
use tasdcailloux::models::element::Element;
use std::thread;
use std::error::Error as StdError;
use std::time::Duration;
use chrono::naive;
use chrono::prelude::*;

lazy_static! {
    pub static ref SOCKET: Mutex<Socket> = {
        let mut socket = Socket::new(Protocol::Req).unwrap();
        socket.connect(&"tcp://127.0.0.1:5555").expect("Fail to bind to tcp port");
        //socket.set_linger(-1).expect("cannot set linger");
        //socket.set_send_timeout(200).unwrap();
        //socket.set_receive_timeout(200).unwrap();
        socket.set_receive_max_size(-1).unwrap();
        Mutex::new(socket)
    };
}

pub fn get_socket() -> &'static SOCKET{
    &SOCKET
}

pub fn check_available() -> bool {
    let mut pollfd_vec: Vec<PollFd> = Vec::new();
    pollfd_vec.push(get_socket().lock().unwrap().new_pollfd(PollInOut::InOut));
    let mut poll_req = PollRequest::new(&mut pollfd_vec[..]);
    let timeout = 10;
    match {let _poll_result = Socket::poll(&mut poll_req, timeout);
           let fds = poll_req.get_fds();
           fds[0].can_write()
    }{
        true => {
            let message = Message{ message_type: AppMessageType::IsReady };
            let encoded: Vec<u8> = serialize(&message, Infinite).unwrap();
            get_socket().lock().unwrap().write(&encoded).unwrap();
            thread::sleep(Duration::from_millis(10));
            let _poll_result = Socket::poll(&mut poll_req, timeout);
            let fds = poll_req.get_fds();
            fds[0].can_read()
        },
        false => false
    }
}

fn ask_and_get_answer (message: Message) -> Result<Response, Error> {
    let encoded: Vec<u8> = serialize(&message, Infinite).unwrap();
    let mut socket = get_socket().lock().unwrap();
    socket.write(&encoded)
        .map_err(|e| Error::SocketWriteError{description: String::from(e.description())})
        .and_then( |_| {
            let mut msg = Vec::new();
            socket.read_to_end(&mut msg)
                .map_err(|e| Error::SocketReadError{description: String::from(e.description())})
                .and_then(|_| {
                    Ok(deserialize(&msg).unwrap())
                })
        })
}

pub fn get_origin_list() -> Result<Vec<Element>, Error>{
    let message = Message{ message_type: AppMessageType::GetAll };
    ask_and_get_answer(message)
        .and_then( |response| {
            match response.response_type {
                ResponseType::GetAll{data} => data,
                _ => Ok(Vec::new())
            }
        })
}

pub fn get_last_updates(since: naive::NaiveDateTime) -> Result<ListCache, Error>{
    let message = Message{ message_type: AppMessageType::GetLastUpdates{since} };
    ask_and_get_answer(message)
        .and_then( |response| {
            match response.response_type {
                ResponseType::GetLastUpdates{data} => data,
                _ => Ok(ListCache{list: Vec::new(), timestamp: NaiveDate::from_ymd(1970, 1, 1).and_hms_milli(0, 0, 0, 42)})
            }
        })
}

pub fn get_one(id: i32) -> Result<Element, Error>{
    let message = Message{ message_type: AppMessageType::GetOne{id} };
    ask_and_get_answer(message)
        .and_then( |response| {
            match response.response_type {
                ResponseType::GetOne{data} => data,
                _ => Err(Error::ElementNotFound)
            }
        })
}

pub fn get_image_for(id: i32, image: i32) -> Result<Vec<u8>, Error>{
    let message = Message{ message_type: AppMessageType::GetImage{id, image} };
    ask_and_get_answer(message)
        .and_then( |response| {
            match response.response_type {
                ResponseType::GetImage{data} => data,
                _ => Ok(Vec::new())
            }
        })
}
