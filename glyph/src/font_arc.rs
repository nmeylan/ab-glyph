use crate::*;
use alloc::sync::Arc;
use core::fmt;

/// `Font` implementor that wraps another concrete `Font + 'static` type storing in an `Arc`.
///
/// Provides convenient type erasure & cheap clones (particularly for `FontVec`).
///
/// # Example
/// ```
/// use ab_glyph::{Font, FontArc, FontVec};
///
/// # fn main() -> Result<(), ab_glyph::InvalidFont> {
/// # let font_data = include_bytes!("../../dev/fonts/Exo2-Light.otf").to_vec();
/// let font_vec = FontVec::try_from_vec(font_data)?;
/// let font_arc = FontArc::from(font_vec);
///
/// assert_eq!(font_arc.descent(), -201.0);
/// # Ok(()) }
/// ```
#[derive(Clone)]
pub struct FontArc(Arc<dyn Font>);

impl FontArc {
    #[inline]
    pub fn new<F: Font + 'static>(font: F) -> Self {
        Self(Arc::new(font))
    }

    /// Creates an `FontArc` from owned data.
    ///
    /// # Example
    /// ```
    /// # use ab_glyph::*;
    /// # fn main() -> Result<(), InvalidFont> {
    /// # let owned_font_data = include_bytes!("../../dev/fonts/Exo2-Light.otf").to_vec();
    /// let font = FontArc::try_from_vec(owned_font_data)?;
    /// # Ok(()) }
    /// ```
    #[inline]
    pub fn try_from_vec(data: Vec<u8>) -> Result<Self, InvalidFont> {
        Ok(FontVec::try_from_vec(data)?.into())
    }

    /// Creates an `FontArc` from a byte-slice.
    ///
    /// # Example
    /// ```
    /// # use ab_glyph::*;
    /// # fn main() -> Result<(), InvalidFont> {
    /// let font = FontArc::try_from_slice(include_bytes!("../../dev/fonts/Exo2-Light.otf"))?;
    /// # Ok(()) }
    /// ```
    #[inline]
    pub fn try_from_slice(data: &'static [u8]) -> Result<Self, InvalidFont> {
        Ok(FontRef::try_from_slice(data)?.into())
    }
}

impl fmt::Debug for FontArc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FontArc")
    }
}

impl Font for FontArc {
    #[inline]
    fn ascent(&self) -> f32 {
        self.0.ascent()
    }

    #[inline]
    fn descent(&self) -> f32 {
        self.0.descent()
    }

    #[inline]
    fn line_gap(&self) -> f32 {
        self.0.line_gap()
    }

    #[inline]
    fn glyph_id(&self, c: char) -> GlyphId {
        self.0.glyph_id(c)
    }

    #[inline]
    fn h_advance(&self, id: GlyphId) -> f32 {
        self.0.h_advance(id)
    }

    #[inline]
    fn h_side_bearing(&self, id: GlyphId) -> f32 {
        self.0.h_side_bearing(id)
    }

    #[inline]
    fn kern(&self, first: GlyphId, second: GlyphId) -> f32 {
        self.0.kern(first, second)
    }

    #[inline]
    fn outline(&self, glyph: GlyphId) -> Option<Outline> {
        self.0.outline(glyph)
    }

    #[inline]
    fn glyph_count(&self) -> usize {
        self.0.glyph_count()
    }
}

impl From<FontVec> for FontArc {
    #[inline]
    fn from(font: FontVec) -> Self {
        Self::new(font)
    }
}
impl From<FontRef<'static>> for FontArc {
    #[inline]
    fn from(font: FontRef<'static>) -> Self {
        Self::new(font)
    }
}
impl From<Arc<dyn Font>> for FontArc {
    #[inline]
    fn from(font: Arc<dyn Font>) -> Self {
        Self(font)
    }
}