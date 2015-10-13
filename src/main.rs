extern crate rustless;
extern crate hyper;
extern crate iron;
extern crate rustc_serialize;
extern crate valico;

use hyper::status::StatusCode;
use iron::Iron;
use rustless::{
    Application, Api, Nesting, Versioning
};
use rustc_serialize::json::ToJson;

fn main() {

  println!("Starting RPI Management ...");

  let api = Api::build(|api| {

    api.post("shutdown", |endpoint| {
        endpoint.handle(|client, params| {
            println!("Shutdown requested");
            client.empty()
        })
    });
  });

  let app = Application::new(api);

  Iron::new(app).http("localhost:4000").unwrap();

}
