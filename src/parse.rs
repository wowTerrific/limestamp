//! This may not be needed or may be used
//! for future versions. Figuring Daylight
//! savings time in addition to offset will
//! be tricky and will require more robust
//! data structures.

use super::Result;
use super::error;
use super::common_times::SECONDS_IN_HOUR;

pub fn parse_offset(original: f32) -> Result<i64> {
    if original > 14.0 || original < -14.0 {
        return Err(Box::new(error::BadOffset {
            message: "You must enter an offset between -14.0 and 14.0".to_string(),
        }))
    }

    // TODO: convert hours, minutes separately 
    todo!();
}

#[cfg(test)]
mod parse_test {
    use super::*;

    #[test]
    fn parse_offset_ending_in_zero() {
        let expected: i64 = 5 * 60 * 60;
        let result = parse_offset(5.0).expect("parse_offset should not error with argument '5.0'");

        assert_eq!(expected, result);
    }

    #[test]
    fn parse_offset_ending_in_seventy_five() {
        let expected: i64 = -((9 * 60 * 60) + 45);
        let result = parse_offset(-9.75).expect("parse_offset should not error with argument '5.0'");

        assert_eq!(expected, result);
    }

    #[test]
    fn parse_offset_ending_in_twenty_five() {
        let expected: i64 = (11 * 60 * 60) + 15;
        let result = parse_offset(11.25).expect("parse_offset should not error with argument '5.0'");

        assert_eq!(expected, result);
    }
}