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
            hours = parse_hour_digits(&offset, 1)?;
            minutes = parse_min_digits(&offset, 3)?;
        },
        6 => {
            hours = parse_hour_digits(&offset, 2)?;
            minutes = parse_min_digits(&offset, 4)?;
        }
        _ => return Err(Box::new(return_error))
    };

    let final_offset = sign * (hours + minutes) as i64;
    Ok(final_offset)
}


fn parse_hour_digits(offset_str: &String, length: usize) -> Result<u64> {
    let return_error = error::BadOffset {
        message: String::from("UTC Offset must be formatted as \"+00:00\", \"-00:00\", \"+0:00\", or \"-0:00\""),
    };

    match length {
        1 => {
            if let Some(hr_char) = offset_str.chars().nth(1) {
                if let Some(digit) = hr_char.to_digit(10) {
                    return Ok(digit as u64 * common_times::SECONDS_IN_HOUR);
                } else {
                    return Err(Box::new(return_error));
                }
            } else {
                return Err(Box::new(return_error));
            }
        },
        2 => {
            if let Ok(hrs_digits) = &offset_str[1..3].parse::<u64>() {
                if hrs_digits > &14 {
                    return Err(Box::new(return_error));
                }
                Ok(*hrs_digits * common_times::SECONDS_IN_HOUR)
            } else {
                return Err(Box::new(return_error));
            }
        },
        _ => return Err(Box::new(return_error))
    }
}



fn parse_min_digits(offset_str: &String, start_index: usize) -> Result<u64> {
    let return_error = error::BadOffset {
        message: String::from("UTC Offset must be formatted as \"+00:00\", \"-00:00\", \"+0:00\", or \"-0:00\""),
    };

    if let Ok(digits) = &offset_str[start_index..].parse::<u64>() {
        match digits {
            00 => return Ok(0),
            15 => return Ok(15 * 60),
            30 => return Ok(30 * 60),
            45 => return Ok(45 * 60),
            _ => return Err(Box::new(return_error))
        }
    } else {
        return Err(Box::new(return_error));
    }
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