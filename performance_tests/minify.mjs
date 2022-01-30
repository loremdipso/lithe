import * as path from 'path';
import * as fs from 'fs';
import { minify } from 'minify';

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
			case "--filename":
				filename = process.argv[i+1];
				break;
		}
	}

	if (!filename) {
		console.log("ERROR: missing filename");
		return;
	}

	const options = {
		js: {
			ecma: 2015,
			/*
			compress: {
				// these did very little
				booleans_as_integers: true,
				keep_fargs: false,

				// these did nothing
				passes: 10,
				unsafe: true,
			}
			*/
		}
	}

	const minified = await minify(filename, options);
	console.log(minified);
}

await main();
