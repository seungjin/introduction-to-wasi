mod bindings;

use anyhow::{anyhow, Result};
use bindings::wasi::http::outgoing_handler;
use bindings::wasi::http::types::{
    Fields, IncomingRequest, Method, RequestOptions, ResponseOutparam, Scheme,
};

struct Component;

bindings::export!(Component with_types_in bindings);

impl bindings::exports::wasi::cli::run::Guest for Component {
    fn run() -> Result<(), ()> {
        let fields = Fields::new();
        let outgoing_request = outgoing_handler::OutgoingRequest::new(fields);
        outgoing_request.set_method(&Method::Get).unwrap();
        outgoing_request.set_scheme(Some(&Scheme::Https)).unwrap();
        outgoing_request.set_authority(Some("ifconfig.io")).unwrap();
        outgoing_request
            .set_path_with_query(Some("/all.json"))
            .unwrap();

        let options = RequestOptions::new();
        let future_response =
            outgoing_handler::handle(outgoing_request, Some(options)).unwrap();

        let incoming_response = match future_response.get() {
            Some(result) => result
                .map_err(|()| anyhow!("response already taken"))
                .unwrap(),
            None => {
                let pollable = future_response.subscribe();
                pollable.block();

                future_response
                    .get()
                    .expect("incoming response available")
                    .map_err(|()| anyhow!("response already taken"))
                    .unwrap()
            }
        };

        let mut body: Vec<u8> = Vec::new();
        let incoming_body = incoming_response.unwrap().consume().unwrap();
        let input_stream = incoming_body.stream().unwrap();

        loop {
            let item = match input_stream.read(1024) {
                Ok(x) => x,
                Err(_) => break,
            };
            if item.is_empty() {
                break;
            }
            for i in item.into_iter() {
                body.push(i);
            }
        }

        println!("{}", String::from_utf8(body).unwrap());
        //println!("rasars");
        Ok(())
    }
}

impl bindings::exports::wasi::http::incoming_handler::Guest for Component {
    fn handle(_request: IncomingRequest, _outparam: ResponseOutparam) {
        // let fields = Fields::new();
        // let outgoing_request = outgoing_handler::OutgoingRequest::new(fields);
        // outgoing_request.set_method(&Method::Get).unwrap();
        // outgoing_request.set_scheme(Some(&Scheme::Https)).unwrap();
        // outgoing_request.set_authority(Some("ifconfig.io")).unwrap();
        // outgoing_request
        //     .set_path_with_query(Some("/all.json"))
        //     .unwrap();

        // let options = RequestOptions::new();

        // let future_response =
        //     outgoing_handler::handle(outgoing_request, Some(options)).unwrap();
        // let incoming_response = match future_response.get() {
        //     Some(result) => result
        //         .map_err(|()| anyhow!("response already taken"))
        //         .unwrap(),
        //     None => {
        //         let pollable = future_response.subscribe();
        //         pollable.block();

        //         future_response
        //             .get()
        //             .expect("incoming response available")
        //             .map_err(|()| anyhow!("response already taken"))
        //             .unwrap()
        //     }
        // };

        // let mut body: Vec<u8> = Vec::new();
        // let incoming_body = incoming_response.unwrap().consume().unwrap();
        // let input_stream = incoming_body.stream().unwrap();

        // loop {
        //     let item = match input_stream.read(1024) {
        //         Ok(x) => x,
        //         Err(_) => break,
        //     };
        //     if item.is_empty() {
        //         break;
        //     }
        //     for i in item.into_iter() {
        //         body.push(i);
        //     }
        // }

        //println!("{}", String::from_utf8(body).unwrap());
    }
}
