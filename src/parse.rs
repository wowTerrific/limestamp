//! This may not be needed or may be used
//! for future versions. Figuring Daylight
//! savings time in addition to offset will
//! be tricky and will require more robust
//! data structures.

use std::string::ParseError;

use super::Result;
use super::error;
use super::common_times::SECONDS_IN_HOUR;

pub fn parse_offset(offset_str: &str) -> Result<i64> {
    // Offset should be 5 or 6 characters long, contain
    // either '+' or '-' as first character
    // one or two digits after (less than 14)
    // ':' next
    // two more digits as: 00, 15, 30, or 45

    match offset_str.len() {
        5 => {
            unimplemented!()
        },
        6 => {
            unimplemented!()
        }
        _ => return Err(Box::new( error::BadOffset { 
                message: String::from("UTC Offset must be formatted as \"+00:00\", \"-00:00\", \"+0:00\", or \"-0:00\"")
            }))
    }
}

#[cfg(test)]
mod parse_test {
    use super::*;

    #[test]
    fn parse_offset_ending_in_zero() {
        let expected: i64 = 5 * 60 * 60;
        let result = parse_offset("+5:00").expect("parse_offset should not error with argument '5.0'");

        assert_eq!(expected, result);
    }

    #[test]
    fn parse_offset_ending_in_forty_five() {
        let expected: i64 = -((9 * 60 * 60) + 45);
        let result = parse_offset("-9:45").expect("parse_offset should not error with argument '5.0'");

        assert_eq!(expected, result);
    }

    #[test]
    fn parse_offset_ending_in_fifteen() {
        let expected: i64 = (11 * 60 * 60) + 15;
        let result = parse_offset("+11:15").expect("parse_offset should not error with argument '5.0'");

        assert_eq!(expected, result);
    }
    #[test]
    fn parse_offset_ending_in_30() {
        let expected: i64 = (8 * 60 * 60) + 30;
        let result = parse_offset("+8:30").expect("parse_offset should not error with argument '5.0'");

        assert_eq!(expected, result);
    }
}