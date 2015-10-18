extern crate rustless;
extern crate hyper;
extern crate iron;
extern crate rustc_serialize;
extern crate valico;

use iron::Iron;
use rustless::{
    Application, Api, Nesting, Versioning
};
use rustless::server::header::AccessControlAllowOrigin;
use rustless::server::status::StatusCode;
use rustc_serialize::json::ToJson;
use std::process::Command;

fn main() {

    println!("Starting RPI Management ...");

    let api = Api::build(|api| {

        api.get("ping", |endpoint| {
            endpoint.handle(|mut client, params| {
                client.set_header(AccessControlAllowOrigin::Any);
                client.empty()
            })
        });

        api.post("shutdown", |endpoint| {
            endpoint.handle(|mut client, params| {
                println!("Shutdown requested...");
                client.set_header(AccessControlAllowOrigin::Any);

                let output = Command::new("shutdown")
                    .arg("-h")
                    .arg("now")
                    .output()
                    .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });

                if output.status.success() {
                    let str_stdout = String::from_utf8(output.stdout).unwrap();
                    println!("Shutdown accepted: {}", str_stdout);

                    client.set_status(StatusCode::NoContent);
                    client.empty()
                } else {
                    let str_stderr = String::from_utf8(output.stderr).unwrap();
                    println!("Shutdown rejected: {}", str_stderr);

                    client.unauthorized();
                    client.text(str_stderr)
                }
            })
        });
    });

    let app = Application::new(api);

    Iron::new(app).http("localhost:4000").unwrap();

}
