# BrainJIT

Your friendly neighborhood [Brainfuck](https://en.wikipedia.org/wiki/Brainfuck) x64 JIT.

## Performance

We got a 17.5x speedup from basic interpretation to full JIT with optimizations!

_Note: many more optimizations could be applied_.

Tested on the classic `mandelbrot.bf`:

| Mode            | No Optimizations | Combine Increments | Replace Sets | Combine Sets |
| --------------- | ---------------- | ------------------ | ------------ | ------------ |
| **Interpreted** | 8.58s            | 3.81s              | 3.73s        | 3.669s       |
| **JIT**         | 1.844s           | 0.527s             | 0.502s       | 0.488s       |

## How to Use

```
Usage: brainjit.exe [OPTIONS] --path <PATH>

Options:
  -m, --mode <MODE>            [default: jit] [possible values: jit, interpret]
  -p, --path <PATH>            The file to run
  -o, --optimize               Optimize the program
  -d, --dump-binary            Dump the binary to a file. Only works in compiled mode
  -t, --tape-size <TAPE_SIZE>  The number of cells in the tape [default: 30000]
  -h, --help                   Print help
  -V, --version                Print version
```
