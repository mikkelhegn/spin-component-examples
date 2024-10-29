## Creating a component

0. Create a WIT file

```wit
package component:business-logic;

interface data-handler {
  // The function to pass on the my-object to.
  handle-data: func(key: my-object) -> my-object;

  // The my-object record, aka. the object to pass between the two components.
  record my-object {
    name: string,
    steps: u32,
    processed: option<bool>,
  }
}

world business-logic {
  export data-handler;
}
```

1. Write the implementation

!Use wit-bindgen, not wit-bindgen-rt

2. (optional) Publish to registry (WIT and Component)

## Using the component from a Spin app

1. Add Dependency and Generate Bindings

```shell
spin deps add ../business-logic/target/wasm32-unknown-unknown/debug/businesslogic.wasm

[Answer to questions...]
```

Creates the `.wit` directory, let's explore it...


```shell
spin deps generate-bindings -L rust -o src/bindings -c spin-app-dependencies
```

Given the component id is "spin-app-dependencies"
