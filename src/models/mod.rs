pub mod element;
use diesel::result::Error as DieselError;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Message {
    pub message_type : MessageType
}


#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum MessageType{
    GetOne{id: i32},
    GetRange{from: i32, to: i32},
    GetAll
}


#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum Error{
    InternalError,
    NotFound,
}
