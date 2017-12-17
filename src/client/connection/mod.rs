use nanomsg::{Socket, Protocol};

pub fn establish_connection() -> Socket {
    let mut socket = Socket::new(Protocol::Req).unwrap();
    socket.connect(&"tcp://127.0.0.1:5555").expect("Fail to bind to tcp port");
    socket
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
