# Solfège

Solfège ("Music Theory" in French) is the Maestro operating system's default booting system.



## Building

To build, simply use the command:

```
cargo build --release
```

For cross compilation, use the `--target` flag with the appropriate target triplet.



### Dependencies

The following C libraries are required:
- libc
- libunwind



## Documentation

Documentation can be found in the book, which can be built using the command:

```
mdbook build book/
```

The book is then accessible at the path `book/book/index.html`.
