use anyhow::Result;
use bytes::Bytes;
use spin_sdk::redis_component;
use std::str::from_utf8;

/// A simple Spin Redis component.
#[redis_component]
fn on_message(message: Bytes) -> Result<()> {
    println!("I'm a Redis handler!");
    println!("{}", from_utf8(&message)?);
    Ok(())
}
