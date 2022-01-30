extern crate web_sys;
use wasm_bindgen::prelude::*;
use log::{LevelFilter, Record, Metadata};
use std::sync::Once;
mod compiler;
use compiler::compiler::compile;

pub fn time_function<T, F: FnMut() -> T>(mut func: F, _label: &str) -> T {
	// TODO: figure out how to do performance testing in WASM.
	// Right now it does not work at all on wasm (depends on window.performance)
	// and is innacurate besides
	return func();
}

#[wasm_bindgen]
pub fn wasm_compile_file(contents: &str) -> String {
	// TODO: is there a better place for this?
	// setup_logging(LevelFilter::Off);
	setup_logging(LevelFilter::Info);

	let result = compile(&contents);
	return result;
}

// thanks https://rustwasm.github.io/book/game-of-life/debugging.html !
// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! wasm_puts {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

static INIT: Once = Once::new();
static LOGGER: WasmLogger = WasmLogger;

/// Setup function that is only run once, even if called multiple times.
fn setup_logging(max_level: LevelFilter) {
    INIT.call_once(|| {
		log::set_logger(&LOGGER)
			.map(|()| log::set_max_level(max_level)).unwrap();
	});
}

struct WasmLogger;
impl log::Log for WasmLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
		// this is handled by max_level above
		return true;
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            wasm_puts!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}
