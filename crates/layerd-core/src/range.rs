//! Byte range primitives for addressing canonical Markdown source text.
//!
//! All persisted core ranges are **half-open byte ranges** (`[start, end)`)
//! into valid UTF-8 source text (RFC-002). Both boundaries must lie on UTF-8
//! character boundaries before any slice or replacement is performed.

/// Errors raised by range construction or validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RangeError {
    /// `start` is greater than `end`.
    Inverted { start: usize, end: usize },
    /// The range extends past the end of the source text.
    OutOfBounds { end: usize, len: usize },
    /// A boundary does not fall on a UTF-8 character boundary.
    NotCharBoundary { offset: usize },
}

impl std::fmt::Display for RangeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RangeError::Inverted { start, end } => {
                write!(f, "inverted range: start {start} > end {end}")
            }
            RangeError::OutOfBounds { end, len } => {
                write!(f, "range end {end} exceeds source length {len}")
            }
            RangeError::NotCharBoundary { offset } => {
                write!(f, "offset {offset} is not a UTF-8 character boundary")
            }
        }
    }
}

impl std::error::Error for RangeError {}

/// A half-open `[start, end)` byte range into canonical UTF-8 source text.
///
/// Invariant: `start <= end`. Validity against a concrete source text
/// (bounds and character boundaries) is checked by [`ByteRange::validate_in`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ByteRange {
    pub start: usize,
    pub end: usize,
}

impl ByteRange {
    /// Creates a range, rejecting inverted bounds.
    pub fn new(start: usize, end: usize) -> Result<Self, RangeError> {
        if start > end {
            return Err(RangeError::Inverted { start, end });
        }
        Ok(Self { start, end })
    }

    /// An empty range positioned at `offset`.
    pub fn empty_at(offset: usize) -> Self {
        Self {
            start: offset,
            end: offset,
        }
    }

    /// Length in bytes.
    pub fn len(&self) -> usize {
        self.end - self.start
    }

    /// Whether the range covers zero bytes.
    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }

    /// Whether `other` is fully contained in `self`.
    pub fn contains_range(&self, other: &ByteRange) -> bool {
        self.start <= other.start && other.end <= self.end
    }

    /// Validates the range against a concrete source text: ordered bounds,
    /// in-bounds end, and both boundaries on UTF-8 character boundaries.
    pub fn validate_in(&self, source: &str) -> Result<(), RangeError> {
        if self.start > self.end {
            return Err(RangeError::Inverted {
                start: self.start,
                end: self.end,
            });
        }
        if self.end > source.len() {
            return Err(RangeError::OutOfBounds {
                end: self.end,
                len: source.len(),
            });
        }
        if !source.is_char_boundary(self.start) {
            return Err(RangeError::NotCharBoundary { offset: self.start });
        }
        if !source.is_char_boundary(self.end) {
            return Err(RangeError::NotCharBoundary { offset: self.end });
        }
        Ok(())
    }

    /// Converts to a standard `Range<usize>` for slicing.
    pub fn as_range(&self) -> std::ops::Range<usize> {
        self.start..self.end
    }
}

impl From<std::ops::Range<usize>> for ByteRange {
    fn from(r: std::ops::Range<usize>) -> Self {
        Self {
            start: r.start,
            end: r.end,
        }
    }
}
