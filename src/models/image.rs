#[derive(Queryable)]
#[derive(PartialEq, Debug)]
#[derive(Serialize, Deserialize)]
pub struct Image {
    pub id: i32,
    pub dir: String
}
