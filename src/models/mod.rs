pub mod element;
pub mod image;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Message {
    pub message_type : MessageType
}


#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum MessageType{
    IsReady,
    GetOne{id: i32},
    GetRange{from: i32, to: i32},
    GetAll,
    GetCount,
    GetImagesCount{id: i32},
    GetImage{id: i32, image: i32}
}


#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum Error{
    InternalError,
    ElementNotFound,
    ImageNotFound
}
