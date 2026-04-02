# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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

[Unreleased]: https://github.com/RosieTheGhostie/maybe-fatal/compare/main%40{1day}...HEAD
