pub use ariadne::{Color, ColorGenerator};

/// A selection of colors to use when building [diagnostic](crate::Diagnostic)s.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ColorPalette {
    /// The primary color to highlight diagnostics with when there is not a more specific option.
    pub special: Color,

    /// Colors with no assigned meaning.
    pub random: [Color; Self::N_RANDOM_COLORS],
}

impl ColorPalette {
    /// The number of colors in [`Self::random`].
    pub const N_RANDOM_COLORS: usize = 4;

    /// The default value of [`Self::random`].
    pub const DEFAULT_RANDOM_COLORS: [Color; Self::N_RANDOM_COLORS] = {
        // This is by no means the most flexible approach, but `core::array::from_fn` isn't stable
        // as a `const` function yet, so this is basically the best we can do right now.
        let mut r#gen = ColorGenerator::new();
        [r#gen.next(), r#gen.next(), r#gen.next(), r#gen.next()]
    };

    /// The default value of [`Self::special`].
    pub const DEFAULT_SPECIAL_COLOR: Color = Color::Fixed(81);

    /// Constructs a new [`ColorPalette`] using the provided color generator.
    ///
    /// Builder methods can be chained with this one to further customize the colors.
    pub const fn new() -> Self {
        Self {
            special: Self::DEFAULT_SPECIAL_COLOR,
            random: Self::DEFAULT_RANDOM_COLORS,
        }
    }

    /// Changes the [special](Self::special) color.
    pub const fn with_special(&mut self, color: Color) -> &mut Self {
        self.special = color;
        self
    }

    /// Changes the [random colors](Self::random).
    pub const fn with_random_colors(
        &mut self,
        colors: [Color; Self::N_RANDOM_COLORS],
    ) -> &mut Self {
        self.random = colors;
        self
    }

    /// Regenerates the [random colors](Self::random) using the provided color generator.
    pub fn with_regenerated_random_colors(&mut self, color_gen: &mut ColorGenerator) -> &mut Self {
        self.with_random_colors(core::array::from_fn(|_| color_gen.next()))
    }
}

impl Default for ColorPalette {
    fn default() -> Self {
        Self::new()
    }
}
