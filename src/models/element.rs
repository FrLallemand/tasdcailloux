use chrono::naive;

#[derive(Queryable)]
#[derive(PartialEq, Debug)]
#[derive(Serialize, Deserialize)]
pub struct Element {
    pub id: i32,
    pub name: String,
    pub weight: i32,
    pub identified: i32,
    pub exposed: i32,
    pub trivia: String,
    pub last_updated: naive::NaiveDateTime
}
