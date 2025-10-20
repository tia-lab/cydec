# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.0.2](https://github.com/tia-lab/cydec/compare/v0.0.1...v0.0.2) - 2025-10-20

### Fixed

- update token retrieval method in release PR workflow for consistency

### Other

- update script formatting in Makefile.toml for better readability
- add complete PR workflow for creating and merging release pull requests
- enhance documentation for compression library with detailed examples and supported types

## [0.0.1](https://github.com/tia-lab/cydec/releases/tag/v0.0.1) - 2025-10-20

### Added

- benchmark tests for improved readability and consistency
- comprehensive tests for integer and floating-point codecs

### Fixed

- correct version number from "0.1.0" to "0.0.1" in Cargo.toml
- update magic bytes from "ORSO" to "CYDEC" in codec headers

### Other

- add cargo-husky for Git hooks and pre-commit checks
- expand README.md with detailed usage, supported types, and performance notes
- add Apache and MIT licenses to the project
- Update README.md
- integer codec with LZ4 compression and decompression
- Initial commit
