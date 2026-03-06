// Real WASM initialization using wasm-bindgen
let wasmLoadingPromise = null;

async function initWasm() {
  if (wasmLoadingPromise) {
    return wasmLoadingPromise;
  }
  
  wasmLoadingPromise = new Promise(async (resolve, reject) => {
    try {
      console.log('Loading Keycloak WASM module with wasm-bindgen...');
      
      // Mark as loading
      window.keycloakWasmAuth = { loading: true, error: null };
      
      // Load the generated JavaScript bindings
      const wasmModule = await import('/wasm/keycloak_wasm_wrapper.js');
      
      // Initialize the WASM module
      await wasmModule.default('/wasm/keycloak_wasm_wrapper.wasm');
      
      // The module should now be available globally
      window.keycloakWasmAuth = {
        loading: false,
        error: null,
        module: wasmModule
      };
      
      console.log('Keycloak WASM module loaded successfully');
      console.log('Available exports:', Object.keys(wasmModule));
      resolve();
      
    } catch (error) {
      console.error('Failed to load WASM module:', error);
      window.keycloakWasmAuth = {
        loading: false,
        error: error,
        module: null
      };
      reject(error);
    }
  });
  
  return wasmLoadingPromise;
}

// Initialize when the page loads
document.addEventListener('DOMContentLoaded', initWasm);

// Also expose the init function globally so components can wait for it
window.initKeycloakWasm = initWasm;