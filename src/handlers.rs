use rustc_serialize::json;

use dotenv::dotenv;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use router::Router;
use iron::prelude::{Request, Response, IronResult};
use iron::status;
use std::env;

use schema::garden::dsl::*;
use models::Garden;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

// A serializable response containing zero or more gardens
#[derive(RustcDecodable, RustcEncodable)]
struct GardensResponse {
    gardens: Vec<Garden>,
}

pub fn garden_handler(req: &mut Request) -> IronResult<Response> {

    // Check if a garden_name is present in the query and store this
    // in optional variable garden_name_opt
    let ref garden_name_opt = req.extensions.get::<Router>()
        .unwrap().find("query");

    println!("Get garden {}", garden_name_opt.unwrap_or("_all"));

    let connection = establish_connection();

    // If a garden_name was supplied, look for this in the database,
    // else return all gardens
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
