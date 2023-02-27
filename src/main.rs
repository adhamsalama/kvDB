use core::panic;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use blitzkrieg::http::{BodyType, HTTPMethod, Request, Response};
use blitzkrieg::server::Server;

fn main() {
    let dict: HashMap<String, String> = HashMap::new();
    let shared_dict = Arc::new(Mutex::new(dict));
    let server = Server::new(
        "127.0.0.1:3000",
        Box::new(move |request| {
            let cloned_dict = Arc::clone(&shared_dict);
            let mut mydict = cloned_dict.lock().unwrap();
            if request.method == HTTPMethod::GET {
                let path = request.path;
                let query = path.split("/").collect::<Vec<&str>>()[1];
                let default_value = String::from("");
                let result = mydict.get(query).unwrap_or(&default_value);
                Response::new(200).body(result)
            } else if request.method == HTTPMethod::POST {
                match request.body.unwrap() {
                    BodyType::FormdataBody(_) => Response::new(400).body("Unsupported Operation."),
                    BodyType::Text(body) => {
                        let operation = body.split("\n").collect::<Vec<&str>>();
                        let key = operation[0];
                        let value = operation[1];
                        mydict.insert(key.into(), value.into());
                        Response::new(200)
                    }
                }
            } else if request.method == HTTPMethod::DELETE {
                let path = request.path;
                let query = path.split("/").collect::<Vec<&str>>()[1];
                mydict.remove(query);
                Response::new(200)
            } else {
                Response::new(400).body("Unsupported Operation.")
            }
        }),
    );
    server.start();
}
