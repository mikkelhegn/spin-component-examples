## Example 1: Importing functionality from a Wasm component in a Spin component

In this example a Spin component (a wasi-http component using the Spin SDK) imports a function from a Wasm component to handle some business logic. The intention with the example is to show how the Spin component can wrap the concern of the transport protocol (HTTP) and pass on only the paylod to another component, which then implements the business logic.

![diagram with two components showing how one exports and interface and the other imports it](./diagram.png)

The [Wasm component (business-logic)](./business-logic/) implements the interface defined in it's [wit files](./business-logic/wit/world.wit):

```wit
interface handle-data {
  // The function to pass on the my-object to.
  handle-data: func(key: my-object) -> my-object;

  // The my-object record, aka. the object to pass between the two components.
  record my-object {
    name: string,
    steps: u32,
    processed: option<bool>,
  }
}
```

The interface includes a function, which takes an argument of the record (type) `my-object`, and returns the same record (type).

Both components are written in Rust, and relies on Cargo to bootstrap `wit-bindgen` for generating bindigns for Rust (from the wit files), on build. `cargo component build` can be use to build both of the components. The full build pipeline to build each component and compose them together is setup in the [`spin.toml` file](./spin-app/spin.toml).

```toml
command = "cargo component build && cargo component build --manifest-path ../business-logic/Cargo.toml && wasm-tools compose target/wasm32-wasi/debug/spinhttpcomponent.wasm -d ../business-logic/target/wasm32-wasi/debug/businesslogic.wasm -o service.wasm"
```

The resulting Wasm module `service.wasm` is a wasi-http compliant module, and can be run by other runtimes than Spin, e.g. Wasmtime.
