[![.github/workflows/build.yml](https://github.com/gregl83/monover/actions/workflows/build.yml/badge.svg)](https://github.com/gregl83/monover/actions/workflows/build.yml)
[![Crates.io](https://img.shields.io/crates/v/monover.svg)](https://crates.io/crates/monover)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/gregl83/monover/blob/master/LICENSE)

# monover

Blazing fast intelligent [monorepo](https://monorepo.tools/) continuous integration versioning.

## State

In-Development of Beta

## Features

### Parallelized Repository Scans

Large file system directory structures are scanned using [Rust Rayon](https://github.com/rayon-rs/rayon) to minimize scan times.

### Version Validations

Package versions are validated to prevent duplicate target versions and variance from versioning scheme.

### Automatic Repository Versioning

Repository versions are automatically derived from package versions.

### Supported Version Schemes

- [Semantic Versioning](https://semver.org/)

### Multilingual Version Fluency

Supports versioning of repository packages using standard package management configuration files. 

See [target.json](targets.json) for complete list of supported files.

Cannot find your favorite package manager? Open an [issue](https://github.com/gregl83/monover/issues/new) to request support.

## Concepts

### CHANGE Files

These files are used for `monover` to decide how to increment package version files.

####  Major
```json
"major"
```

#### Minor
```json
"minor"
```

#### Patch
```json
"patch"
```

#### Pre-Release
```json
"<major|minor>-<pre-release-name>"
```

### VERSION Files

These are special files that have three main functions:

- Version repository (auto-generated).
- Version repository packages (overrides version file matches).
- Settle version ambiguity (1 < version file match).

The format for `VERSION` files is a simple double-quoted string. They are valid JSON files without file extensions.

There are two possible values for the `VERSION` file string.

#### Versioning Scheme Version

```json
"1.0.0"
```

#### Version File Key

Must match a file key in [targets.json](targets.json).

```json
"cargo.toml"
```

Use this option for version ambiguity due to multiple files in [targets.json](targets.json) found in single package.

## License

[MIT](LICENSE)
