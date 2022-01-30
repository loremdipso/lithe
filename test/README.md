# Summary

Tests the svelte and lithe compilers.

# Setup

This relies on [github_batch_download](https://github.com/loremdipso/github_batch_download/) to fetch svelte files to test against. There's more info in that project's readme, but basically you can change into that directory and run:

```
cargo run --release -- --language svelte --license mit --limit 100
```

To download up to 100 svelte repos with a permissive MIT license. Note that it helps to add a personal access token to stave off github's rate limiting.

# scripts/extract.rb {argument}

| Argument           | Description                                                                                                     |
| ------------------ | --------------------------------------------------------------------------------------------------------------- |
| all                | run all subcommands                                                                                             |
| extract            | copies the svelte files from `github_batch_downloader/output/` into `files/cleaned/`                            |
| dedup              | removes duplicates files from `files/cleaned/`                                                                  |
| compile            | uses the svelte compiler to compile all the files in `files/cleaned/` and puts the results in `files/compiled/` |
| minify             | minifies all the files in `files/compiled/` and puts the results in `files/minified/`                           |
| compile_and_minify | compiles and minifies                                                                                           |
| gzip               | gzips all the files in `files/minified/` and puts the results in `files/gzip/`                                  |

NOTE: this is pretty slow. Might want to make yourself a cuppa :)

# scripts/stats.rb

Runs some basic statistics over your `files/` directory
