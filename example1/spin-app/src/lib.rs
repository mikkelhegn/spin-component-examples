use spin_sdk::http::{IntoResponse, Params, Request, Response, Router};
use spin_sdk::http_component;

mod bindings;
use bindings::component::business_logic::handle_data::{handle_data, MyObject};

/// A simple Spin HTTP component.
#[http_component]
fn handle_http(req: Request) -> Response {
    let mut router = Router::new();
    router.post("/", handle_post);
    router.any("/*", handle_any);
    router.handle(req)
}

fn handle_post(req: Request, _params: Params) -> anyhow::Result<impl IntoResponse> {
    let my_object: MyObject = serde_json::from_slice(req.body())?;
    let handled_data = handle_data(&my_object);

    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body(serde_json::to_string(&handled_data)?)
        .build())
}

fn handle_any(_req: Request, _params: Params) -> anyhow::Result<impl IntoResponse> {
    Ok(Response::builder().status(501).build())
}
