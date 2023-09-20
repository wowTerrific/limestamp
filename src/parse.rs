//! This may not be needed or may be used
//! for future versions. Figuring Daylight
//! savings time in addition to offset will
//! be tricky and will require more robust
//! data structures.

use std::string::ParseError;

use crate::common_times;

use super::Result;
use super::error;
use super::common_times::SECONDS_IN_HOUR;

pub fn parse_offset(offset_str: &str) -> Result<i64> {
    // Offset should be 5 or 6 characters long, contain
    // either '+' or '-' as first character
    // one or two digits after (less than 14)
    // ':' next
    // two more digits as: 00, 15, 30, or 45

    let offset = String::from(offset_str);

    let return_error = error::BadOffset {
        message: String::from("UTC Offset must be formatted as \"+00:00\", \"-00:00\", \"+0:00\", or \"-0:00\""),
    };

    let mut sign: i64 = 1;
    let hours: u64;
    let minutes: u64;

    if let Some(c) = &offset.chars().nth(0) {
        if c == &'+' {
            sign = 1;
        } else if c == &'-' {
            sign = -1;
        } else {
            return Err(Box::new(return_error));
        }
    }

    match offset.len() {
        5 => {
            if let Some(hr_char) = offset.chars().nth(1) {
                if let Some(digit) = hr_char.to_digit(10) {
                    hours = digit as u64 * common_times::SECONDS_IN_HOUR;
                } else {
                    return Err(Box::new(return_error));
                }
            } else {
                return Err(Box::new(return_error));
            }

            if let Ok(digit) = (&offset[3..]).parse::<u64>() {
                match digit {
                    00 => minutes = 0,
                    15 => minutes = 15 * 60,
                    30 => minutes = 30 * 60,
                    45 => minutes = 45 * 60,
                    _ => return Err(Box::new(return_error))
                }
            } else {
                return Err(Box::new(return_error))
            }

        
        },
        6 => {
            unimplemented!()
        }
        _ => return Err(Box::new(return_error))
    };

    let final_offset = sign * (hours + minutes) as i64;
    Ok(final_offset)
}

#[cfg(test)]
mod parse_test {
    use super::*;

    #[test]
    fn parse_offset_ending_in_zero() {
        let expected: i64 = 5 * 60 * 60;
        let result = parse_offset("+5:00").expect("parse_offset should not error with argument '+5:00'");

        assert_eq!(expected, result);
    }

    #[test]
    fn parse_offset_ending_in_forty_five() {
        let expected: i64 = -((9 * 60 * 60) + (45 * 60));
        let result = parse_offset("-9:45").expect("parse_offset should not error with argument '-9:45'");

        assert_eq!(expected, result);
    }

    #[test]
    fn parse_offset_ending_in_fifteen() {
        let expected: i64 = (11 * 60 * 60) + (15 * 60);
        let result = parse_offset("+11:15").expect("parse_offset should not error with argument '+11:15'");

        assert_eq!(expected, result);
    }
    #[test]
    fn parse_offset_ending_in_thirty() {
        let expected: i64 = (8 * 60 * 60) + (30 * 60);
        let result = parse_offset("+8:30").expect("parse_offset should not error with argument '5.0'");

        assert_eq!(expected, result);
    }
}