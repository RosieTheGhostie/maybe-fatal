use ariadne::ReportBuilder;

/// Some contextual information about a [diagnostic](crate::Diagnostic).
///
/// Consumers of this crate generally should not need to explicitly use this type. It is only made
/// public to provide the ability to [filter](crate::sink::Filter) diagnostics by their context on
/// the off-chance that is necessary.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Context<S> {
    /// A labeled subspan of a [diagnostic](crate::Diagnostic), optionally with an attached message.
    Label(ariadne::Label<S>),

    /// A message that provides specialized information concerning the overall
    /// [diagnostic](crate::Diagnostic).
    Note(Box<str>),

    /// Advice concerning the overall [diagnostic](crate::Diagnostic).
    Help(Box<str>),
}

impl<S> Context<S> {
    /// Constructs a new note.
    ///
    /// If you already have a [`Box<str>`], you could also construct the note directly from the
    /// [`Note`](Context::Note) variant.
    pub fn new_note(note: impl ToString) -> Self {
        Self::new_note_owned(note.to_string())
    }

    /// Constructs a new note from an owned [`String`].
    pub fn new_note_owned(note: String) -> Self {
        Self::Note(note.into_boxed_str())
    }

    /// Constructs a new help message.
    ///
    /// If you already have a [`Box<str>`], you could also construct the help message directly from
    /// the [`Help`](Context::Help) variant.
    pub fn new_help(help: impl ToString) -> Self {
        Self::new_help_owned(help.to_string())
    }

    /// Constructs a new help message from an owned [`String`].
    pub fn new_help_owned(help: String) -> Self {
        Self::Help(help.into_boxed_str())
    }

    /// Adds this context to a [`ReportBuilder`].
    pub(super) fn add_to_report_builder(self, builder: &mut ReportBuilder<S>)
    where
        S: ariadne::Span,
    {
        match self {
            Self::Label(label) => builder.add_label(label),
            Self::Note(note) => builder.add_note(note),
            Self::Help(help) => builder.add_help(help),
        }
    }
}
