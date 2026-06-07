//! RFC-002 range unit policy: half-open byte ranges, ordered bounds,
//! in-bounds ends, and UTF-8 character boundary validation.

use crate::{ByteRange, RangeError};

#[test]
fn new_rejects_inverted_bounds() {
    assert_eq!(
        ByteRange::new(5, 3),
        Err(RangeError::Inverted { start: 5, end: 3 })
    );
    assert!(ByteRange::new(3, 3).is_ok());
    assert!(ByteRange::new(3, 5).is_ok());
}

#[test]
fn validate_rejects_out_of_bounds_end() {
    let source = "abc";
    let range = ByteRange { start: 0, end: 4 };
    assert_eq!(
        range.validate_in(source),
        Err(RangeError::OutOfBounds { end: 4, len: 3 })
    );
}

#[test]
fn validate_rejects_non_char_boundaries_in_multibyte_text() {
    // "あ" is 3 bytes (E3 81 82); offsets 1 and 2 split the character.
    let source = "あい";
    for bad in [1usize, 2] {
        let range = ByteRange { start: bad, end: 3 };
        assert_eq!(
            range.validate_in(source),
            Err(RangeError::NotCharBoundary { offset: bad })
        );
    }
    // Offsets 0, 3, 6 are valid boundaries.
    assert!(ByteRange { start: 0, end: 3 }.validate_in(source).is_ok());
    assert!(ByteRange { start: 3, end: 6 }.validate_in(source).is_ok());
}

#[test]
fn ranges_are_half_open() {
    let source = "hello";
    let range = ByteRange { start: 1, end: 3 };
    assert_eq!(&source[range.as_range()], "el");
    assert_eq!(range.len(), 2);
    assert!(!range.is_empty());
    assert!(ByteRange::empty_at(2).is_empty());
}

#[test]
fn containment_is_inclusive_of_equal_bounds() {
    let outer = ByteRange { start: 2, end: 10 };
    assert!(outer.contains_range(&ByteRange { start: 2, end: 10 }));
    assert!(outer.contains_range(&ByteRange { start: 4, end: 6 }));
    assert!(!outer.contains_range(&ByteRange { start: 1, end: 6 }));
    assert!(!outer.contains_range(&ByteRange { start: 4, end: 11 }));
}
