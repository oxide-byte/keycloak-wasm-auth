# Keycloak WASM Auth

## Motivation

Currently there are a couple of libraries already existing on the crates.io. One of them https://crates.io/crates/leptos-keycloak-auth/0.3.1 also took my attention and will most probably be my choice for a SSR application.

And this was one of my first motivations. I apply Leptos currently in CSR and generated a special Use Case to communicate in GraphQL (https://github.com/oxide-byte/rust-berlin-leptos). And for doing my tasks, I had started to build this module and extracted it now here.

## React-JS

A sample to implement the Rust-WASM with a Wrapper in an React-JS application:

samples/keycloak-react

The build can be found under:

samples/wasm-wrapper/README.md

## Quickstart

From the samples:

Starting Docker-Compose

```shell
docker compose -f samples/docker/demo_hnl/docker-compose.yaml up
```

Starting React-JS

```shell
cd samples/keycloak-react
npm run dev
```

Starting Leptos

```shell
cd samples/keycloak-csr-leptos
trunk serve --port 8080
```

**Attention**

Leptos and React-JS cannot run in the same time, they share the port 8080 for the application

## There will be more soon...