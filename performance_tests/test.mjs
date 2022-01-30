import * as path from 'path';
import * as fs from 'fs';
import svelte from '../svelte/compiler.js';
import { minify } from 'minify';

function timeFunction(callback, label, config) {
	//try {
	// prime the pump before doing any performance calculation
	let _dryrun = callback();

	let start = Date.now();
	const result = callback();
	let end = Date.now();
	console.log(`=================== ${label}: ${end - start}ms`);
	if (config.show_result) {
		console.log(result);
	}
	//} catch (e) {
	//console.log(`=================== ${label}: FAILURE`);
	//}
}

async function main() {
	let do_time = false;
	let do_minify = false;
	let do_svelte = false;
	let only_js = false;
	let show_result = false;
	let filename = null;
	for (let i = 0; i < process.argv.length; i++) {
		let arg = process.argv[i];
		switch (arg) {
			case "--time":
				do_time = true;
				break;
			case "--only_js":
				only_js = true;
				break;
			case "--lithe":
				do_lithe = true;
				break;
			case "--svelte":
				do_svelte = true;
				break;
			case "--minify":
				do_minify = true;
				break;
			case "--filename":
				filename = process.argv[i+1];
				break;
			case "--show_result":
			case "--show_output":
				show_result = true;
				break;
		}
	}

	// svelte config
	const svelte_function = async () => {
		let result = svelte.compile(contents);
		if (only_js) {
			result = result.js.code;
		} else {
			result = result;
		}

		if (do_minify) {
			const output_filename = "temp.js";
			fs.writeFileSync(output_filename, result);
			const minified = await minify(output_filename);
			return minified;
		} else {
			return result;
		}
	}

	if (!filename) {
		console.log("ERROR: missing filename");
		return;
	}

	const contents = fs.readFileSync(filename, 'utf-8');
	const config = { show_result, do_minify };

	if (do_time) {
		timeFunction(svelte_function, "original", config);
	} else {
		try {
			console.log(await svelte_function(config));
		} catch {
			console.log("ERROR: some error in compilation");
		}
	}
}

await main();
