// This lib version builds a native Rust executable
// that we could reference via FFI. For now let's instead use WASM
extern crate libc;
use libc::c_char;
use log::{info, LevelFilter, Record, Metadata};
use std::ffi::{CStr, CString};
use std::sync::Once;
use std::time::Instant;
mod compiler;
use compiler::compiler::compile;

pub fn time_function<T, F: FnMut() -> T>(mut func: F, label: &str) -> T {
	let now = Instant::now();
	let rv = func();
	info!("{}: {}ms", label, now.elapsed().as_millis());
	return rv;
}

fn setup_logging_wrapper() {
	// TODO: is this the best place for this?
	// native::setup_logging(LevelFilter::Off);
	setup_logging(LevelFilter::Info);
}

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

static INIT: Once = Once::new();
static LOGGER: NativeLogger = NativeLogger;

/// Setup function that is only run once, even if called multiple times.
fn setup_logging(max_level: LevelFilter) {
    INIT.call_once(|| {
		log::set_logger(&LOGGER)
			.map(|()| log::set_max_level(max_level)).unwrap();
	});
}

struct NativeLogger;
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
