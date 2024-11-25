use wasi::{
    cli::stderr::OutputStream,
    http::types::{
        Fields, IncomingRequest, Method, OutgoingBody, OutgoingResponse,
        ResponseOutparam,
    },
};

struct HttpService;

wasi::http::proxy::export!(HttpService);

impl wasi::exports::http::incoming_handler::Guest for HttpService {
    fn handle(request: IncomingRequest, response_out: ResponseOutparam) {
        let hdrs = Fields::new();
        let resp = OutgoingResponse::new(hdrs);
        let body = resp.body().expect("outgoing response");

        ResponseOutparam::set(response_out, Ok(resp));

        let out = body.write().expect("outgoing stream");
        out.blocking_write_and_flush(b"Hello, wasi:http/proxy world!\n")
            .expect("writing response");

        drop(out);
        OutgoingBody::finish(body, None).unwrap();
    }
}
