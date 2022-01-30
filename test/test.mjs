// import * as path from 'path';
import * as fs from 'fs';
import svelte from '../svelte/compiler.js';
// import { minify } from 'minify';
import ffi from 'ffi-napi';
import { wasm_compile_file } from '../lib/wasm/lithe.js';
import { parse } from 'node-html-parser';

function timeFunction(callback, label, config) {
	// Prime the pump before doing any performance calculation
	// This still isn't terribly accurate, but at least we can ignore initialization costs
	callback();

	let start = Date.now();
	const result = callback();
	let end = Date.now();

	console.log(`=================== ${label}: ${end - start}ms`);
	if (config.show_result) {
		console.log(result);
	}
}

async function main() {
	let do_time = false;
	let do_lithe_native = false;
	let do_lithe_wasm = false;
	let do_simple_html_parser = false;
	let do_svelte = false;
	let only_js = false;
	let filename = null;

	// only useful if release_mode is enabled.
	// This also runs the debug version
	let do_debug = false;

	let release_mode = false;
	let show_result = false;
	for (let i = 0; i < process.argv.length; i++) {
		const arg = process.argv[i];
		switch (arg) {
			case "--native":
				do_lithe_native = true;
				break;
			case "--time":
				do_time = true;
				break;
			case "--wasm":
				do_lithe_wasm = true;
				break;
			case "--simple_html_parser":
				do_simple_html_parser = true;
				break;
			case "--only_js":
				only_js = true;
				break;
			case "--debug":
				do_lithe_native = true;
				do_debug = true;
				break;
			case "--svelte":
				do_svelte = true;
				break;
			case "--release":
				do_lithe_native = true;
				release_mode = true;
				break;
			case "--file":
				filename = process.argv[i + 1];
				break;
			case "--show_result":
			case "--show_output":
				show_result = true;
				break;
		}
	}

	if (!filename) {
		console.error("ERROR: need to provide a filename. --file [file]");
		return;
	}

	// timing defaults to all if none were specified
	if (do_time && !(do_svelte || do_lithe_native || do_lithe_wasm || do_simple_html_parser)) {
		do_lithe_native = true;
		do_lithe_wasm = true;
		do_svelte = true;
	}

	// const minified = await minify('./output.js');
	// console.log(minified);

	// svelte config
	const svelte_function = () => {
		let result = svelte.compile(contents);
		if (only_js) {
			return result.js.code;
		} else {
			return result;
		}
	}

	// lithe config
	const lithe_wasm_function = () => {
		let result = wasm_compile_file(`${contents}`);
		result = JSON.parse(result);
		if (only_js) {
			return result.js.code;
		} else {
			return result;
		}
	};

	const simple_html_parser = () => {
		const root = parse(contents);
		// root.firstChild.tagName = "div";
		// const fake = root.toString();
		// return fake;
	};

	const lithe_native_function = (() => {
		// don't load ffi if we aren't going to use it
		if (do_lithe_native) {
			const lib = ffi.Library(process.env.HOME + `/.cargo-target/${release_mode ? 'release' : 'debug'}/liblithe.so`, {
				// TODO: is the best way to test this via 'compile_string', or more like 'compile_file'?
				// or some other thing?
				compile_string: ['string', ['string']]
			});

			return () => {
				let result = lib.compile_string(`${contents}`);
				// TODO: can we just return the thing and not worry about serialization/etc?
				result = JSON.parse(result);
				if (only_js) {
					return result.js.code;
				} else {
					return result;
				}
			};
		} else {
			return () => {
				throw new Error("This shouldn't happen :)");
			}
		}
	})();

	const contents = fs.readFileSync(filename, 'utf-8');

	const config = {
		show_result,
	};

	if (do_time) {
		if (do_svelte) {
			timeFunction(
				svelte_function,
				"original",
				config
			);
		}
		if (do_simple_html_parser) {
			timeFunction(
				simple_html_parser,
				"simple html parser",
				config
			);
		}
		if (do_lithe_native) {
			timeFunction(
				lithe_native_function,
				`lithe - native (${release_mode ? "RELEASE" : "DEBUG"})`,
				config
			);
		}
		if (release_mode && do_debug) {
			release_mode = false; // TODO: remove, it's hacky
			timeFunction(
				lithe_native_function,
				`lithe - native (DEBUG)`,
				config
			);
			release_mode = true;
		}
		if (do_lithe_wasm) {
			timeFunction(
				lithe_wasm_function,
				"lithe - WASM",
				config
			);
		}
	} else {
		if (do_svelte) {
			console.log(svelte_function());
		}
		if (do_lithe_native) {
			console.log(lithe_native_function());
		}
		if (do_lithe_wasm) {
			console.log(lithe_wasm_function());
		}
	}
}

await main();
