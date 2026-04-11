# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- 'minimal-tao' example program.
- 'tao' example program.
- `message` method for `DiagnosticGroup` trait.
- `into_diagnostic` method for `DiagnosticGroupExt` trait.
- Blanket implementation of `Diagnose<S, D>` for `(S, T)` where `T` implements `DiagnosticGroup<D>`
  and `PartialDiagnose<S, D>`.
- Builder API for `sink::Report`.
  - `from_sources` constructor method.
  - `set_config` method.
  - `with_config` method.
- Ability to specify diagnostic messages, notes, and help messages through derive API.

### Changed

- Builder API for `sink::Filter`.
  - Rename `with_add_callback` method to `set_add_callback`.
  - Add new `with_add_callback` method that consumes `self`.
- Make `label` meta attribute of `maybe_fatal` helper use list syntax in order to support format
  strings.

### Removed

- `DiagnosticMessageResolver` trait in favor of `DiagnosticGroup::message` method.
- `config` parameter of `sink::Report::new` in favor of new builder API.

### Fixed

- Outdated version in repository-level 'README.md'.

## [0.1.0-beta.3] - 2026-04-10

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

[Unreleased]: https://github.com/RosieTheGhostie/maybe-fatal/compare/v0.1.0-beta.3...HEAD
[0.1.0-beta.3]: https://github.com/RosieTheGhostie/maybe-fatal/compare/v0.1.0-beta.2...v0.1.0-beta.3
[0.1.0-beta.2]: https://github.com/RosieTheGhostie/maybe-fatal/compare/v0.1.0-beta.1...v0.1.0-beta.2
[0.1.0-beta.1]: https://github.com/RosieTheGhostie/maybe-fatal/releases/tag/v0.1.0-beta.1
