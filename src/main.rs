extern crate iron;
extern crate router;
extern crate diesel;
extern crate dotenv;
extern crate cog;
extern crate rustc_serialize;

use iron::prelude::{Iron, Request, Response, IronResult};
use iron::status;
use router::Router;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use rustc_serialize::json;

use self::cog::schema::garden::dsl::*;
use self::cog::models::Garden;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

#[derive(RustcDecodable, RustcEncodable)]
struct GardensResponse {
    gardens: Vec<Garden>,
}

fn garden_handler(req: &mut Request) -> IronResult<Response> {
    let ref garden_name_opt = req.extensions.get::<Router>()
        .unwrap().find("query");

    println!("Get garden {}", garden_name_opt.unwrap_or("_all"));

    let connection = establish_connection();

    match *garden_name_opt {
        Some(garden_name) => {
            let result = garden.filter(name.eq(garden_name))
                .first::<Garden>(&connection)
                .expect("Error loading garden");
            Ok(Response::with((status::Ok, result.name.to_string())))
        },
        None => {
            let results = garden
                .limit(10)
                .load::<Garden>(&connection)
                .expect("Error loading gardens");
            let response = GardensResponse{ gardens: results };
            Ok(Response::with((status::Ok, json::encode(&response).unwrap())))
        }
    }
}

fn main() {
    let mut router = Router::new();
    router.get("/api/v1/garden/", garden_handler, "garden_all");
    router.get("/api/v1/garden/:name/", garden_handler, "garden_by_name");

    Iron::new(router).http("localhost:3000").unwrap();

}
