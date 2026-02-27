# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/),
and this project adheres to [Semantic Versioning](https://semver.org/).

## [Unreleased]

### Added

- Core trait definitions: `SzEngine`, `SzConfig`, `SzConfigManager`, `SzDiagnostic`, `SzProduct`, `SzAbstractFactory`
- `SzError` enum with 16 error variants and hierarchical `SzErrorKind` classification
- Bitmask flag constants mirroring the C SDK headers
- Common parameter constants (`SZ_NO_LOGGING`, `SZ_VERBOSE_LOGGING`, etc.)
- CI workflow with build, test, clippy, and format checks
