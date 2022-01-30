// This builds either WASM or Native assumbly.

#[cfg(not(feature = "wasm"))]
extern crate libc;
#[cfg(not(feature = "wasm"))]
use libc::c_char;
#[cfg(not(feature = "wasm"))]
use std::ffi::{CStr, CString};
#[cfg(not(feature = "wasm"))]
use std::time::Instant;
#[cfg(not(feature = "wasm"))]
use log::info;


#[cfg(feature = "wasm")]
extern crate web_sys;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

use log::{LevelFilter, Record, Metadata};
use std::sync::Once;
mod compiler;
use compiler::compiler::compile;

static INIT: Once = Once::new();

#[cfg(feature = "wasm")]
static LOGGER: WasmLogger = WasmLogger;
#[cfg(not(feature = "wasm"))]
static LOGGER: NativeLogger = NativeLogger;

/// Setup function that is only run once, even if called multiple times.
fn setup_logging(max_level: LevelFilter) {
    INIT.call_once(|| {
		log::set_logger(&LOGGER)
			.map(|()| log::set_max_level(max_level)).unwrap();
	});
}

fn setup_logging_wrapper() {
	// TODO: is this the best place for this?
	// native::setup_logging(LevelFilter::Off);
	setup_logging(LevelFilter::Info);
}

#[cfg(feature = "wasm")]
pub fn time_function<T, F: FnMut() -> T>(mut func: F, _label: &str) -> T {
	// TODO: figure out how to do performance testing in WASM.
	// Right now it does not work at all on wasm (depends on window.performance)
	// and is innacurate besides
	return func();
}

#[cfg(not(feature = "wasm"))]
pub fn time_function<T, F: FnMut() -> T>(mut func: F, label: &str) -> T {
	let now = Instant::now();
	let rv = func();
	info!("{}: {}ms", label, now.elapsed().as_millis());
	return rv;
}

#[cfg(not(feature = "wasm"))]
#[no_mangle]
pub extern "C" fn compile_string(s: *const c_char) -> *mut c_char {
	setup_logging_wrapper();
    let c_str = unsafe {
        assert!(!s.is_null());
        CStr::from_ptr(s)
    };

    let r_str = c_str.to_str().unwrap();
	let result = compile(&r_str);
	return CString::new(result).unwrap().into_raw();
}

#[cfg(not(feature = "wasm"))]
#[no_mangle]
pub extern "C" fn compile_file(path: *const c_char) -> *mut c_char {
	setup_logging_wrapper();
    let c_str = unsafe {
        assert!(!path.is_null());
        CStr::from_ptr(path)
    };

    let path = c_str.to_str().unwrap();
	let contents = std::fs::read_to_string(path).unwrap();
	let result = compile(&contents);
	// TODO: make sure we aren't leaking memory. I mean, we are (on purpose), but
	// make sure Node is freeing it like it should.
	return CString::new(result).unwrap().into_raw();
}

#[wasm_bindgen]
#[cfg(feature = "wasm")]
pub fn wasm_compile_file(contents: &str) -> String {
	setup_logging_wrapper();
	let result = compile(&contents);
	return result;
}

#[cfg(not(feature = "wasm"))]
struct NativeLogger;
#[cfg(not(feature = "wasm"))]
impl log::Log for NativeLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
		// this is handled by max_level above
		return true;
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

// thanks https://rustwasm.github.io/book/game-of-life/debugging.html !
// A macro to provide `println!(..)`-style syntax for `console.log` logging.
#[cfg(feature = "wasm")]
macro_rules! wasm_puts {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[cfg(feature = "wasm")]
struct WasmLogger;
#[cfg(feature = "wasm")]
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
