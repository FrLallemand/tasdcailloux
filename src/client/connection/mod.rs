use std::io::{Read, Write};
use std::sync::Mutex;

use nanomsg::{Socket, Protocol,  PollFd, PollRequest, PollInOut};
use bincode::{serialize, deserialize, Infinite};
use tasdcailloux::models::{Message, Error};
use tasdcailloux::models::MessageType as AppMessageType;
use tasdcailloux::models::element::Element;
use std::thread;
use std::time::Duration;

lazy_static! {
    pub static ref SOCKET: Mutex<Socket> = {
        let mut socket = Socket::new(Protocol::Req).unwrap();
        socket.connect(&"tcp://127.0.0.1:5555").expect("Fail to bind to tcp port");
        //socket.set_linger(-1).expect("cannot set linger");
//        socket.set_send_timeout(200).unwrap();
 //       socket.set_receive_timeout(200).unwrap();
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

pub fn get_origin_list() -> Result<Vec<Element>, Error>{
    let message = Message{ message_type: AppMessageType::GetAll };
    let encoded: Vec<u8> = serialize(&message, Infinite).unwrap();
    match get_socket().lock().unwrap().write(&encoded) {
        Ok(_) => {
        },
        Err(_) => {
        }
    };

    let mut msg = Vec::new();
    match get_socket().lock().unwrap().read_to_end(&mut msg) {
        Ok(_) => {
            deserialize(&msg).unwrap()
        },
        Err(_) => {
            Err(Error::InternalError)
        }
    }
}

pub fn get_one(id: i32) -> Result<Element, Error>{
    let message = Message{ message_type: AppMessageType::GetOne{id} };
    let encoded: Vec<u8> = serialize(&message, Infinite).unwrap();

    match get_socket().lock().unwrap().write(&encoded) {
        Ok(_) => {
        },
        Err(_) => {
        }
    };

    let mut msg = Vec::new();
    match get_socket().lock().unwrap().read_to_end(&mut msg) {
        Ok(_) => {
            deserialize(&msg).unwrap()
        },
        Err(_) => {
            Err(Error::InternalError)
        }
    }
}

pub fn get_image_for(id: i32, image: i32) -> Result<Vec<u8>, Error>{
    let message = Message{ message_type: AppMessageType::GetImage{id, image} };
    let encoded: Vec<u8> = serialize(&message, Infinite).unwrap();

    match get_socket().lock().unwrap().write(&encoded) {
        Ok(_) => {
        },
        Err(_) => {
        }
    };

    let mut msg = Vec::new();
    match get_socket().lock().unwrap().read_to_end(&mut msg) {
        Ok(_) => {
            let img_result: Result<Vec<u8>, Error> = deserialize(&msg).unwrap();
            match img_result {
                Ok(img) => {
                    //let mut file = File::create(format!("{}_{}", id.to_string(), image.to_string())).unwrap();
                    //let mut file = File::create("tasqsqssdcailloux").unwrap();
                    //let mut writer = BufWriter::new(file);
                    //writer.write(&img);
                    Ok(img)
                },
                Err(_) => {
                    Err(Error::InternalError)
                }
            }
        },
        Err(e) => {
            println!("{:?}", e);
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
