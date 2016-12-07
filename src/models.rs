#[derive(Queryable, RustcDecodable, RustcEncodable)]
pub struct Garden {
    pub id: i32,
    pub name: String,
    pub description: String,
}

#[derive(Queryable, RustcDecodable, RustcEncodable)]
pub struct Area {
    pub id: i32,
    pub garden_id: i32,
    pub name: String,
}

#[derive(Queryable, RustcDecodable, RustcEncodable)]
pub struct Vertex {
    pub id: i32,
    pub area_id: i32,
    pub x: f32,
    pub y: f32,
    pub ordinal: f32,
}
