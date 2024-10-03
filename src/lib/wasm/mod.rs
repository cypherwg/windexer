use wasmer::{Store, Module, Instance};

pub struct WasmRuntime {
    store: Store,
}

impl WasmRuntime {
    pub fn new() -> Self {
        Self { store: Store::default() }
    }

    pub fn run_custom_indexer(&mut self, wasm_bytes: &[u8], input: &[u8]) -> anyhow::Result<Vec<u8>> {
        let module = Module::new(&self.store, wasm_bytes)?;
        let instance = Instance::new(&mut self.store, &module, &[])?;
        // TODO: custom indexer execution logic
        Ok(vec![])
    }
}