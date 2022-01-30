What's interesting is how the svelte compiler seems to grow linearly with the input size. That is:

| Version | Input Lines | Compile time (ms) |
| ------- | ----------- | ----------------- |
| Svelte  | 1           | 4ms               |
| Svelte  | 100         | 143ms             |
| Svelte  | 1000        | 682ms             |
| Lithe   | 1           | 1ms               |
| Lithe   | 100         | 1ms               |
| Lithe   | 1000        | 7ms               |

Now, while 7ms might not be accurate for what this has the potential to be (I mean, lithe isn't really doing anything at this point, just the absolute bare minimum to support basic tags), over half a second for 1 kloc is really very bad, I feel. And this isn't even counting the file I/O, which is very likely going to be faster in Rust land.
