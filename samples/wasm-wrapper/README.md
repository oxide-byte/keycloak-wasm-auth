To build the wasm file for the react sample, you can use `wasm-pack`. Run the following command from this directory (`samples/wasm-wrapper`):

```shell
wasm-pack build --target web --out-dir ../keycloak-react/public/wasm --no-typescript --out-name keycloak_wasm_wrapper .
```

This will generate `keycloak_wasm_wrapper.js` and `keycloak_wasm_wrapper_bg.wasm` in the output directory. You need to rename the `.wasm` file to match what the React sample expects:

```shell
mv ../keycloak-react/public/wasm/keycloak_wasm_wrapper_bg.wasm ../keycloak-react/public/wasm/keycloak_wasm_wrapper.wasm
```