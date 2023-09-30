use std::time::SystemTime;

use parse::parse_offset;

pub mod error;
mod parse;
pub mod common_times;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Entry point of limestamp. All you need is your UTC Offset.
/// The UTC Offset is a float with times converted to base 10
/// decimals. For example, if you are in NPT or Nepal Time,
/// your offset converts from +5:45 to 5.75. Values above 14
/// below -14 will throw an Error.
/// 
/// The Timestamp will also be set up as MM/DD/YYYY. Sorry about
/// your luck, anyone outside of the US...
pub fn limestamp(offset: &str)-> Result<String> {
    
    let offset_in_seconds = parse::parse_offset(offset)?;

    let date_in_sec = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)?
        .as_secs();

    let adjusted_date_in_sec = if offset_in_seconds > 0 {
        date_in_sec + offset_in_seconds as u64
    } else {
        date_in_sec - (offset_in_seconds.abs() as u64)
    };


    // TODO:
    // cycles - help to calculate leap years!
    // num days - helps to calculate everything else... 

    let num_days = adjusted_date_in_sec / common_times::SECONDS_IN_DAY;
    let cycles = (num_days - (365 + 366)) / common_times::CYCLE_IN_DAYS;
    let remainder_years_after_cycle = (num_days - (365+366)) % common_times::CYCLE_IN_DAYS / 365;

    // Year - 1972 because 1970 is in the middle of a "cycle"
    let year = (cycles * 4) + remainder_years_after_cycle + 1972;

    // Month
    // TODO!
    let remainder_days_after_cycle = 0;

    let month = 0;
    // Day
    let day = 0;



    let hour = ( adjusted_date_in_sec % (60 * 60 * 24) ) / ( 60 * 60 );
    let minute = ( adjusted_date_in_sec % (60 * 60) ) / 60;
    let second = adjusted_date_in_sec % 60;

    let date_as_str = format!("{}/{}/{} - {}:{}:{}", month, day, year, hour, minute, second);

    Ok(date_as_str)
}




// This is the old function - DELETE THIS!!!
fn get_date() -> Result<String> {
    let date_in_sec = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)?
        .as_secs();

    const DAY_MONTH: [u64; 11] = [31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];
    const SECONDS_IN_YEAR: u64 = 31_536_000;
    const SECONDS_IN_DAY: u64 = 86_400;
    const THREE_YEARS_IN_DAYS: u64 = 365 * 3;

    let num_days = date_in_sec / SECONDS_IN_DAY;

    let cycles = (num_days - (365+366)) / (THREE_YEARS_IN_DAYS + 366);
    let remainder_years = ((num_days - (365+366)) % (THREE_YEARS_IN_DAYS + 366)) / 365;
    let year = (cycles * 4) + remainder_years + 1972;
    let remainder_days =  (num_days % (cycles * (THREE_YEARS_IN_DAYS + 366))) % 365;
    let mut month: u64 = 12;
    let mut day: u64 = 31;
    for (i, val) in DAY_MONTH.iter().enumerate() {
        if remainder_days <= *val {
            month = i as u64;   // TODO!
            day = remainder_days % val;
            break;
        }
    }

    let hours = ( date_in_sec % (60 * 60 * 24) ) / ( 60 * 60 );
    let minutes = ( date_in_sec % (60 * 60) ) / 60;
    let seconds = date_in_sec % 60;

    // TODO: Leap years????
    // let years: u64 = (date_in_sec / SECONDS_IN_YEAR) + 1970;
    // let months: u64 = (date_in_sec % SECONDS_IN_YEAR) / (SECONDS_IN_DAY * 30);
    
    // let date_format = format!("Years: {}\nMonths: {}", years, months);
    let date_format = format!("{}/{}/{} - {}:{}:{}", month, day, year, hours, minutes, seconds);
    // let date_format = format!("{}:{}:{}", hours, minutes, seconds);

    Ok(date_format)
}
// DELETE ABOVE!!!!


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_limestamp_string() {
        println!("{:?}", limestamp("-05:00"))
    }

    #[test]
    fn limestamp_bad_offset_error() {
        let lime_result = limestamp("+5:45");

        println!("{lime_result:?}");

        if lime_result.is_ok() {
            
            panic!("limestamp should have thrown BadOffset Error");
        }
    }
}
