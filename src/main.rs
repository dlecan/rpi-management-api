extern crate rustless;
extern crate hyper;
extern crate iron;
extern crate rustc_serialize;
extern crate valico;

use iron::Iron;
use rustless::{
    Application, Api, Nesting, Versioning
};
use rustless::framework::endpoint::*;
use rustless::server::header::AccessControlAllowOrigin;
use rustless::server::status::*;
use rustc_serialize::json::ToJson;
use std::process::Command;

fn main() {

    println!("Starting RPI Management ...");

    let api = Api::build(|api| {

        api.prefix("api");

        api.before(|mut client, _params| {
            client.set_header(AccessControlAllowOrigin::Any);
            Ok(())
        });

        api.namespace("system", |system_ns| {

            system_ns.post("shutdown", |endpoint| {
                endpoint.handle(|mut client, _params| {
                    println!("Shutdown requested...");

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

                        client.internal_server_error();
                        client.text(str_stderr)
                    }
                })
            });

        });

        api.namespace("health", |health_ns| {

            let ping = |endpoint: &mut Endpoint| -> EndpointHandlerPresent {
                endpoint.handle(|client, _params| {
                    client.empty()
                })
            };

            health_ns.get("ping", &ping);
            health_ns.head("ping", &ping);

        });

        api.namespace("hostapd", |hostapd_ns| {

            hostapd_ns.post("start", |endpoint| {
                endpoint.handle(|mut client, _params| {
                    client.not_implemented();
                    client.empty()
                })
            });

            hostapd_ns.post("stop", |endpoint| {
                endpoint.handle(|mut client, _params| {
                    client.not_implemented();
                    client.empty()
                })
            });

        });

        api.namespace("minidlna", |minidlna_ns| {

            minidlna_ns.post("reinit", |endpoint| {
                endpoint.handle(|mut client, _params| {
                    client.not_implemented();
                    client.empty()
                })
            });

        });

    });

    let app = Application::new(api);

    Iron::new(app).http("0.0.0.0:4000").unwrap();

}
