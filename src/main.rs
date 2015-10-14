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
use rustless::server::header::AccessControlAllowOrigin;
use rustc_serialize::json::ToJson;
use std::process::Command;

fn main() {

  println!("Starting RPI Management ...");

  let api = Api::build(|api| {

    api.post("shutdown", |endpoint| {
        endpoint.handle(|mut client, params| {
            println!("Shutdown requested");
            client.set_header(AccessControlAllowOrigin::Any);

            let output = Command::new("shutdown")
                     .arg("-h")
                     .arg("now")
                     .output()
                     .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
            let strStdout = String::from_utf8(output.stdout).unwrap();
            client.text(strStdout)
            // client.empty()
        })
    });
  });

  let app = Application::new(api);

  Iron::new(app).http("localhost:4000").unwrap();

}
