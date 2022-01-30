NOT AT ALL READY TO USE. Presently this serves as a super-experimental testbed. Run at your own risk.

To test the various compilers against test.svelte, you simply need to run test.sh. Options:

| Option                 | Description                                                                                  |
| ---------------------- | -------------------------------------------------------------------------------------------- |
| `--file` \[path\]      | which file should we attempt to compile                                                      |
| `--release`            | also run the native release build                                                            |
| `--debug`              | also run the native debug build                                                              |
| `--debug`              | also run the wasm build                                                                      |
| `--svelte`             | also run the svelte build (assumes it lives at "../svelte/compiler.js"                       |
| `--simple_html_parser` | also run a very basic [JS-based HTML parser](https://www.npmjs.com/package/node-html-parser) |
| `--show_output`        | show the output of each command                                                              |
| `--only_js`            | only show the `code.js` portion of the output                                                |
| `--skip_build`         | don't rebuild before running tests                                                           |
