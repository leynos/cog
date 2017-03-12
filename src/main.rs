extern crate iron;
extern crate router;
extern crate dotenv;
extern crate cog;

use router::Router;
use iron::prelude::Iron;

use self::cog::handlers::garden_handler;

fn main() {
    let mut router = Router::new();
    router.get("/api/v1/garden/", garden_handler, "garden_all");
    router.get("/api/v1/garden/:name/", garden_handler, "garden_by_name");

    Iron::new(router).http("localhost:3000").unwrap();

}
