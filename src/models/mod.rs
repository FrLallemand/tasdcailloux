pub mod element;
pub mod image;

use chrono::naive;
use self::element::Element;

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
    GetImage{id: i32, image: i32},
    GetLastUpdates{since: naive::NaiveDateTime}
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Response {
    pub response_type : ResponseType,
}


#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum ResponseType{
    GetOne{data: Result<Element, Error>},
    GetRange{data: Result<Vec<Element>, Error>},
    GetAll{data: Result<Vec<Element>, Error>},
    GetCount{data: Result<i32, Error>},
    GetImagesCount{data: Result<i32, Error>},
    GetImage{data: Result<Vec<u8>, Error>},
    GetLastUpdates{data: Result<ListCache, Error>}
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct ListCache {
    pub list: Vec<Element>,
    pub timestamp: naive::NaiveDateTime
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum Error{
    InternalError,
    ElementNotFound,
    ImageNotFound,
    SocketReadError{description: String},
    SocketWriteError{description: String}
}
