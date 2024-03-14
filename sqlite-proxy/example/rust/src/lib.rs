use serde::Serialize;
use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::http_component;
use spin_sdk::sqlite;

#[derive(Serialize)]
pub struct Customer {
    id: String,
    name: String,
    email: String,
}

/// A simple Spin HTTP component.
#[http_component]
fn handle_http_handler(_req: Request) -> anyhow::Result<impl IntoResponse> {
    let connection = sqlite::Connection::open("default")?;

    let result = connection.execute("SELECT * FROM customers;", &[])?;

    let customers: Vec<Customer> = result
        .rows()
        .map(|row| Customer {
            id: row.get::<u64>("id").unwrap().to_string(),
            name: row.get::<&str>("name").unwrap().to_owned().to_string(),
            email: row.get::<&str>("email").unwrap().to_owned().to_string(),
        })
        .collect();

    let body = serde_json::to_string(&customers)?;

    Ok(Response::new(200, body))
}
