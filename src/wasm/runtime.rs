use anyhow::Result;
use wasmer::{Instance, Module, Store};
use wasmer_compiler_cranelift::Cranelift;
use std::path::Path;

use crate::utils::error::Error;
use super::types::{WasmInput, WasmOutput};

pub struct WasmRuntime {
    store: Store,
}

impl WasmRuntime {
    pub fn new() -> Self {
        let compiler = Cranelift::default();
        let store = Store::new(compiler);
        Self { store }
    }

    pub fn execute_wasm(&self, wasm_path: &Path, input: WasmInput) -> Result<WasmOutput> {
        let wasm_bytes = std::fs::read(wasm_path)?;
        let module = Module::new(&self.store, wasm_bytes)?;
        let instance = Instance::new(&module, &[])?;

        let process_func = instance.exports.get_function("process")?;
        let result = process_func.call(&[wasmer::Value::I32(input.value)])?;

        let output_value = result[0].unwrap_i32();
        Ok(WasmOutput { value: output_value })
    }
}

impl Default for WasmRuntime {
    fn default() -> Self {
        Self::new()
    }
}