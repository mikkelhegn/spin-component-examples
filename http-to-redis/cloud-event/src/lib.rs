use spin_sdk::http::conversions::FromBody;
use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::http_component;

mod bindings;
use bindings::fermyon::spin::inbound_redis::handle_message;
use bindings::fermyon::spin::redis_types::Payload;

/// A simple Spin HTTP component.
#[http_component]
fn handle_cloud_event(req: Request) -> anyhow::Result<impl IntoResponse> {
    println!("I'm an HTTP handler!");

    let payload = Payload::from_body(req.body().to_vec());

    handle_message(&payload).unwrap();

    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body("Hello, Fermyon")
        .build())
}
