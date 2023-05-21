[![.github/workflows/build.yml](https://github.com/gregl83/monover/actions/workflows/build.yml/badge.svg)](https://github.com/gregl83/monover/actions/workflows/build.yml)
# monover

Blazing fast intelligent [monorepo](https://github.com/gregl83/monorepo) continuous integration versioning.

## Features

### Parallelized Repository Scans

Large file system directory structures are scanned using [Rust Rayon](https://github.com/rayon-rs/rayon) to minimize scan times.

### Version Validations

Package versions are validated to prevent duplicate target versions and variance from versioning scheme.

### Automatic Repository Versioning

Repository versions are automatically derived from package versions.

### Historical Version Log

Keeps track of versions to validate new target versions and perform state audits.

Version Log uses Rust implementation of the [Cap'n Proto](https://github.com/capnproto/capnproto-rust) message format.

### Supported Version Schemes

- [Semantic Versioning](https://semver.org/)

### Multilingual Version Fluency

Supports versioning of repository packages using standard package management configuration files. 

- Generic
  - VERSION
- Go
  - go.mod
- NodeJs
  - package.json
- Rust
  - cargo.toml
- C++
  - conanfile.py
  - conanfile.txt
- Python
  - setup.cfg
  - setup.py
- Java
  - pom.xml
  - build.gradle
- Scala
  - build.sbt
- Swift
  - Package.swift
- Dart
  - pubspec.yaml
- Clojure
  - project.clj
- F#
  - paket.dependencies
- C#
  - .csproj
  - .nuspec
- Haskell
  - .cabal
- Kotlin
  - build.gradle.kts
- R
  - DESCRIPTION
- Lua
  - .rockspec
- OCaml
  - .opam
  - opam
- PHP
  - composer.json
- Perl
  - Makefile.PL
  - Build.PL
  - META.yml
- Julia
  - Project.toml
- Crystal
  - shard.yml
- Nim
  - .nimble
- Erlang
  - rebar.config
- Ruby
  - .gemspec
- Elixir
  - mix.exs
- Elm
  - elm.json
- Idris
  - elba.toml

## Concepts

### CHANGE Files

These files are used for monover to decide how to increment package version files.

Example:

```json
"major"
```

### VERSION Files

These are special files that have three main functions:

- Version repository (auto-generated).
- Version repository packages (overrides version file matches).
- Settle version ambiguity (1 < version file match).

The format for `VERSION` files is a simple double quoted string. They are valid JSON files without file extensions.

There are two possible values for the `VERSION` file string:

- Valid value for Versioning Scheme.
- Key of version file to use.

## License

[MIT](LICENSE)
