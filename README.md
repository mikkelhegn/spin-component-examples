# Spin and the Component Model examples

This repo contains a few examples for the great companionship bwtween Spin and the component model.

The repo examples are provided "as is" with no guarantees :-)

## [Example 1: Importing functionality from a Wasm component in a Spin component](./spin-app-import/README.md)

In this example a Spin component (a wasi-http component using the Spin SDK) imports a function from a Wasm component to handle some business logic. The intention with the example is to show how the Spin component can wrap the concern of the transport protocol (HTTP) and pass on only the paylod to another component, which then implements the business logic. 

## External examples

### [HTTP-Authentication](https://github.com/fermyon/http-auth-middleware)

This repo is an example of how to compose a middleware component with a business logic component.

### [Demo Docker Container for wasi-http](https://github.com/fermyon/wasi-http-tools-demo-container)

This repository contains a Dockerfile, which creates a container to help demo wasi-http across Spin, NGINX Unit, and Wastime.