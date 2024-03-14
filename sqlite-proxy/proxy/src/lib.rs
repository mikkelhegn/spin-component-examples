use exports::fermyon::spin::sqlite::{
    Error, GuestConnection, OwnConnection, QueryResult, RowResult, Value,
};
use regex::Regex;

pub struct ProxyConnection(spin_sdk::sqlite::Connection);

impl GuestConnection for ProxyConnection {
    fn open(database: String) -> Result<OwnConnection, Error> {
        let conn = spin_sdk::sqlite::Connection::open(&database).map(ProxyConnection)?;
        Ok(spin_sdk::wit_bindgen::rt::Resource::new(conn))
    }

    fn execute(&self, statement: String, parameters: Vec<Value>) -> Result<QueryResult, Error> {
        let parameters = parameters.into_iter().map(Into::into).collect::<Vec<_>>();

        // Grab the QueryResult
        let mut query_result: QueryResult =
            self.0.execute(&statement, &parameters).map(From::from)?;

        // Check all rows for a Text column. If it matches the e-mail patter, then obfuscate
        for row_result in query_result.rows.iter_mut() {
            for value in row_result.values.iter_mut() {
                match value {
                    Value::Text(v) => {
                        if is_email(v) {
                            *v = hide_email_domain(v);
                        }
                    }
                    _ => {}
                }
            }
        }

        // Return the QueryResult
        Ok(query_result)
    }
}

fn is_email(email: &str) -> bool {
    // Define a regular expression for a simple email validation
    let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();

    // Check if the email matches the regular expression
    email_regex.is_match(email)
}

fn hide_email_domain(email: &str) -> String {
    // Define a regular expression to extract the domain and TLD
    let email_regex = Regex::new(r"^(?P<local>[^@]+)@(?P<domain>[^\.]+)\.(?P<tld>.+)$").unwrap();

    // Use the regular expression to capture parts of the email
    if let Some(captures) = email_regex.captures(email) {
        // Replace the domain with stars
        let hidden_domain = captures
            .name("domain")
            .map_or("", |m| m.as_str())
            .chars()
            .map(|_| '*')
            .collect::<String>();
        // Retrieve the TLD
        let tld = captures.name("tld").map_or("", |m| m.as_str());
        // Recreate the modified email address
        format!(
            "{}@{}.{}",
            captures.name("local").unwrap().as_str(),
            hidden_domain,
            tld
        )
    } else {
        // If the email doesn't match the expected format, return it unchanged
        email.to_string()
    }
}

impl From<spin_sdk::sqlite::QueryResult> for QueryResult {
    fn from(qr: spin_sdk::sqlite::QueryResult) -> Self {
        QueryResult {
            columns: qr.columns,
            rows: qr.rows.into_iter().map(From::from).collect(),
        }
    }
}

impl From<spin_sdk::sqlite::RowResult> for RowResult {
    fn from(row: spin_sdk::sqlite::RowResult) -> RowResult {
        RowResult {
            values: row.values.into_iter().map(From::from).collect(),
        }
    }
}

impl Into<spin_sdk::sqlite::Value> for Value {
    fn into(self) -> spin_sdk::sqlite::Value {
        match self {
            Value::Integer(v) => spin_sdk::sqlite::Value::Integer(v),
            Value::Real(v) => spin_sdk::sqlite::Value::Real(v),
            Value::Text(v) => spin_sdk::sqlite::Value::Text(v),
            Value::Blob(v) => spin_sdk::sqlite::Value::Blob(v),
            Value::Null => spin_sdk::sqlite::Value::Null,
        }
    }
}

impl From<spin_sdk::sqlite::Value> for Value {
    fn from(val: spin_sdk::sqlite::Value) -> Self {
        match val {
            spin_sdk::sqlite::Value::Integer(v) => Value::Integer(v),
            spin_sdk::sqlite::Value::Real(v) => Value::Real(v),
            spin_sdk::sqlite::Value::Text(v) => Value::Text(v),
            spin_sdk::sqlite::Value::Blob(v) => Value::Blob(v),
            spin_sdk::sqlite::Value::Null => Value::Null,
        }
    }
}

impl From<spin_sdk::sqlite::Error> for Error {
    fn from(e: spin_sdk::sqlite::Error) -> Self {
        match e {
            spin_sdk::sqlite::Error::AccessDenied => Error::AccessDenied,
            spin_sdk::sqlite::Error::NoSuchDatabase => Error::NoSuchDatabase,
            spin_sdk::sqlite::Error::InvalidConnection => Error::InvalidConnection,
            spin_sdk::sqlite::Error::DatabaseFull => Error::DatabaseFull,
            spin_sdk::sqlite::Error::Io(e) => Error::Io(e),
        }
    }
}

wit_bindgen::generate!({
    runtime_path: "::spin_sdk::wit_bindgen::rt",
    world: "proxy",
    path: "wit",
    exports: {
        world: ProxyConnection,
        "fermyon:spin/sqlite/connection": ProxyConnection,
    }
});
