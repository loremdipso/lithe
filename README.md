# Summary and Disclaimer

This is an experimental Svelte compiler written in Rust. It is **NOT AT ALL READY TO USE**. Right now this just serves as a super-experimental testbed. This "compiler" does little more than parse HTML and perform some minor transformations to it. By no means is it feature complete with the Svelte compiler. Run at your own risk.

# Setup

To get this ready for testing simply run `npm install`. This does many things, among them initializing the git submodule that Svelte is referenced from and install/building everything needed to run various performance and correctness tests.

# Testing

To test the various compilers and compiler versions you simply need to run `scripts/test.sh`. Note that the WASM version is always built in release mode.

Options:

| Option                 | Description                                                                                       |
| ---------------------- | ------------------------------------------------------------------------------------------------- |
| `--file` \[path\]      | which file should we attempt to compile, relative to the `test` directory. Default: `test.svelte` |
| `--release`            | also run the native release build                                                                 |
| `--debug`              | also run the native debug build                                                                   |
| `--wasm`               | also run the WASM build                                                                           |
| `--svelte`             | also run the svelte build (assumes it lives at `svelte/compiler.js`                               |
| `--simple_html_parser` | also run a very basic [JS-based HTML parser](https://www.npmjs.com/package/node-html-parser)      |
| `--show_output`        | show the output of each command                                                                   |
| `--only_js`            | only show the `code.js` portion of the output                                                     |
| `--skip_build`         | don't rebuild before running tests                                                                |

# Current State

I've written up my thoughts and findings on [my blog](https://loremdipso.com/tags/lithe/). Presently I think this would ultimately be faster than Svelte's Typescript-base compiler, that WASM is an interesting means of deploying builds, but that with ES6 modules the performance gains are unimportant. Larger projects may benefit, especially when doing full rebuilds, but for local development where you only need to recompile a single file at a time going from `100ms` to `1ms` isn't groundbreaking, in my humble opinion.

But feel free to use this code or any of the ideas here in your own projects. I don't think I've done anything all that novel, but I think sometimes seeing the same ideas rearranged a bit can be helpful.
