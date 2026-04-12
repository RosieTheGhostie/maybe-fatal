//! # Additional Attributes
//!
//! All derive macros for the [`maybe_fatal`](crate) crate use the `maybe_fatal` helper attribute to
//! configure code generation. Depending on the context, specific parts of the attribute may be
//! ignored, required, or inferred.
//!
//! ## Item Attributes
//!
//! These attributes can be applied to `struct`s and `enum`s.
//!
//! - `span_type = ...`
//!
//!   Sets the span type for the generated code.
//!
//!   This is required for most macros.
//!
//! - `group(prefix = "...", discriminant_type = ...)`
//!
//!   Configures group-wide information for the [`DiagnosticGroup`] and [`DiagnosticInfoWrapper`]
//!   macros.
//!
//!   `prefix` is required to be a 3-byte UTF-8 string and must be specified. `discriminant_type` is
//!   optional and defaults to [`code::DefaultDiscriminant`] if not specified.
//!
//! ## Field Attributes
//!
//! These attributes can be applied to the fields of `struct`s and `enum` **variants**.
//!
//! - `info`
//!
//!   Marks this field as containing the information for the [`Diagnose`] macro.
//!
//!   There can only be one field with this attribute.
//!
//! - `span`
//!
//!   Marks this field as the span for the [`Diagnose`] macro.
//!
//!   There can only be one field with this attribute.
//!
//! - `label(...)`
//!
//!   Adds a label to the diagnostic, using the relevant field as the span.
//!
//!   See the section on this attribute's structure for more details.
//!
//! ## Variant Attributes
//!
//! These attributes can be applied to the variants of `enum`s.
//!
//! - `message(...)`, `note(...)`, and `help(...)`
//!
//!   Adds a message of the relevant kind to the diagnostic. `message` can only be set once per
//!   variant, but `note` and `help` are unrestricted.
//!
//!   The contents are interpreted as if they were being passed to the [`format`] macro.
//!
//! - `label(...)`
//!
//!   Adds a label to the entire diagnostic's span.
//!
//!   See the section on this attribute's structure for more details.
//!
//! ## `label(...)` Attribute Structure
//!
//! The label attribute (regardless of where it appears) accepts the following additional
//! attributes.
//!
//! - `message(...)`
//!
//!   Sets the message attached to the label.
//!
//!   The contents are interpreted as if they were being passed to the [`format`] macro.
//!
//! - `color = ...`
//!
//!   Sets the color of the label.
//!
//!   The value is interpreted as an expression, and that expression should evaluate to a
//!   [`Color`]. For convenience, whatever [`ColorPalette`] happens to be available for the relevant
//!   trait implementation will be exposed under the `colors` variable.
//!
//! - `order = ...`
//!
//!   Sets the relative order of the label.
//!
//!   The value is interpreted as an expression, and that expression should evaluate to an [`i32`].
//!
//! [`code::DefaultDiscriminant`]: crate::code::DefaultDiscriminant
//! [`Color`]: ariadne::Color
//! [`ColorPalette`]: crate::ColorPalette
//! [`Diagnose`]: https://docs.rs/maybe-fatal-derive/latest/maybe_fatal_derive/derive.Diagnose.html
//! [`DiagnosticGroup`]: https://docs.rs/maybe-fatal-derive/latest/maybe_fatal_derive/derive.DiagnosticGroup.html
//! [`DiagnosticInfoWrapper`]: https://docs.rs/maybe-fatal-derive/latest/maybe_fatal_derive/derive.DiagnosticInfoWrapper.html
//! [`PartialDiagnose`]: https://docs.rs/maybe-fatal-derive/latest/maybe_fatal_derive/derive.PartialDiagnose.html
