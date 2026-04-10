# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- `derive` macros for creating diagnostic-related types.
  - `Diagnose`
  - `PartialDiagnose`
  - `DiagnosticGroup`
  - `DiagnosticInfo`

## [0.1.0-beta.2] - 2026-04-02

### Added

- Public exports of `ariadne::Color` and `ariadne::ColorGenerator`.
- Continuous integration (CI) via GitHub Actions.

### Fixed

- API ergonomics.

## [0.1.0-beta.1] - 2026-04-02

### Added

- `Diagnostic` and `ClassifiedDiagnostic` types (i.e., the whole point of the library).
- `DiagnosticSeverity` enumeration for classifying diagnostics.
- `Diagnose`, `PartialDiagnose`, and `DiagnosticGroup` traits for building diagnostics from user
  types.
- `DiagnosticMessageResolver` trait for diagnostic message lookup.
- `Sink` trait along with a handful of types that implement it.
  - `Collect`
  - `Filter`
  - `Ignore`
  - `Report`
- Less notable types and traits that are still part of the public API.
  - `Context`
  - `ColorPalette`
  - `code::DefaultDiscriminant`
  - `code::Discriminant`
  - `Lenient`
- This CHANGELOG file.

[Unreleased]: https://github.com/RosieTheGhostie/maybe-fatal/compare/v0.1.0-beta.2...HEAD
[0.1.0-beta.2]: https://github.com/RosieTheGhostie/maybe-fatal/compare/v0.1.0-beta.1...v0.1.0-beta.2
[0.1.0-beta.1]: https://github.com/RosieTheGhostie/maybe-fatal/releases/tag/v0.1.0-beta.1
