<p align="center">
  <picture>
    <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/llenotre/maestro-lnf/master/logo-light.svg">
    <img src="https://raw.githubusercontent.com/llenotre/maestro-lnf/master/logo.svg" alt="logo" width="50%" />
  </picture>
</p>

[![MIT license](https://img.shields.io/badge/license-MIT-blue.svg?style=for-the-badge&logo=book)](./LICENSE)
![Version](https://img.shields.io/badge/dynamic/toml?url=https%3A%2F%2Fraw.githubusercontent.com%2Fllenotre%2Fsolfege%2Fmaster%2FCargo.toml&query=%24.package.version&style=for-the-badge&label=version)
![Continuous integration](https://img.shields.io/github/actions/workflow/status/llenotre/solfege/check.yml?style=for-the-badge&logo=github)



# About

Solfège ("Music Theory" in French) is the Maestro operating system's default booting system.



## Build

To build, simply use the command:

```
cargo build --release
```

For cross compilation, use the `--target` flag with the appropriate target triplet.



## Documentation

Documentation can be found in the book, which can be built using the command:

```
mdbook build book/
```

The book is then accessible at the path `book/book/index.html`.
