# Spin and the Component Model examples

This repo contains a few examples for the great companionship bwtween Spin and the component model.

The repo examples are provided "as is" with no guarantees :-)

## Example 1: Using a Wasm component from a Spin component

In this example a Spin component (a wasi-http component using the Spin SDK) imports a function from a Wasm component to handle some business logic. The intention with the example is to show how the Spin component can wrap the concern of the transport protocol (HTTP) and pass on only the paylod to another component, which then implements the business logic. 
