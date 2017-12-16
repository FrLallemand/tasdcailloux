pub mod element;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Message {
    pub message_type : MessageType
}


#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum MessageType{
    GetOne,
    GetRange{from: i32, to: i32},
    GetAll
}