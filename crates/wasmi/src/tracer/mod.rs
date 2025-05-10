use common::rv_trace::{ELFInstruction, MemoryState, RVTraceRow, RegisterState};
use core::cell::RefCell;
use std::vec::Vec;

#[derive(Debug)]
pub struct Tracer {
    pub rows: RefCell<Vec<RVTraceRow>>,
    open: RefCell<bool>,
}

impl Tracer {
    pub fn new() -> Self {
        Self {
            rows: RefCell::new(Vec::new()),
            open: RefCell::new(false),
        }
    }

    pub fn start_instruction(&self, inst: ELFInstruction) {
        let mut inst = inst;
        inst.address = inst.address as u32 as u64;
        *self.open.try_borrow_mut().unwrap() = true;
        self.rows.try_borrow_mut().unwrap().push(RVTraceRow {
            instruction: inst,
            register_state: RegisterState::default(),
            memory_state: None,
            advice_value: None,
            precompile_input: None,
            precompile_output_address: None,
        });
    }

    pub fn capture_pre_state(&self, reg: [i64; 32]) {
        // if !*self.open.try_borrow().unwrap() {
        //     return;
        // }

        // let mut rows = self.rows.try_borrow_mut().unwrap();
        // let row = rows.last_mut().unwrap();

        // if let Some(rs1) = row.instruction.rs1 {
        //     row.register_state.rs1_val = Some(normalize_register_value(reg[rs1 as usize], xlen));
        // }

        // if let Some(rs2) = row.instruction.rs2 {
        //     row.register_state.rs2_val = Some(normalize_register_value(reg[rs2 as usize], xlen));
        // }
    }

    pub fn capture_post_state(&self, reg: [i64; 32]) {
        // if !*self.open.try_borrow().unwrap() {
        //     return;
        // }

        // let mut rows = self.rows.try_borrow_mut().unwrap();
        // let row = rows.last_mut().unwrap();

        // if let Some(rd) = row.instruction.rd {
        //     row.register_state.rd_post_val = Some(normalize_register_value(reg[rd as usize], xlen));
        // }
    }

    pub fn push_memory(&self, memory_state: MemoryState) {
        // if !*self.open.try_borrow().unwrap() {
        //     return;
        // }

        // if let Some(row) = self.rows.try_borrow_mut().unwrap().last_mut() {
        //     row.memory_state = Some(memory_state);
        // }
    }

    pub fn end_instruction(&self) {
        *self.open.try_borrow_mut().unwrap() = false;
    }
}

#[cfg(test)]
mod tests {

    use crate::{Caller, Engine, Error, Linker, Module, Store};
    use std::{ffi::OsStr, fs, path::Path, println, string::String, vec::Vec};

    /// Returns the contents of the given `.wasm` or `.wat` file.
    ///
    /// # Errors
    ///
    /// If the Wasm file `wasm_file` does not exist.
    /// If the Wasm file `wasm_file` is not a valid `.wasm` or `.wat` format.
    pub fn read_wasm_or_wat(wasm_file: &Path) -> Vec<u8> {
        let mut wasm_bytes = fs::read(wasm_file).unwrap();
        if wasm_file.extension().and_then(OsStr::to_str) == Some("wat") {
            let wat = String::from_utf8(wasm_bytes).unwrap();
            wasm_bytes = wat2wasm(&wat).unwrap()
        }
        wasm_bytes
    }

    /// Converts the given `.wat` into `.wasm`.
    pub fn wat2wasm(wat: &str) -> Result<Vec<u8>, wat::Error> {
        wat::parse_str(wat)
    }

    // In this simple example we are going to compile the below Wasm source,
    // instantiate a Wasm module from it and call its exported "hello" function.
    fn trace() -> Result<(), Error> {
        type HostState = u32;
        let wasm = read_wasm_or_wat(Path::new("./binaries/energy_usage.wasm"));
        // First step is to create the Wasm execution engine with some config.
        //
        // In this example we are using the default configuration.
        let engine = Engine::default();
        // Now we can compile the above Wasm module with the given Wasm source.
        let module = Module::new(&engine, wasm)?;

        // Wasm objects operate within the context of a Wasm `Store`.
        //
        // Each `Store` has a type parameter to store host specific data.
        // In this example the host state is a simple `u32` type with value `42`.
        let mut store = Store::new(&engine, ());

        // A linker can be used to instantiate Wasm modules.
        // The job of a linker is to satisfy the Wasm module's imports.
        let linker = <Linker<()>>::new(&engine);
        let instance = linker.instantiate(&mut store, &module)?.start(&mut store)?;
        // Now we can finally query the exported "hello" function and call it.
        let func = instance.get_func(&store, "main").unwrap();
        let res = func.call(&mut store, ())?;
        Ok(())
    }

    #[test]
    fn test_trace() {
        trace().unwrap();
    }
}
