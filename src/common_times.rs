pub const MONTH_BY_DAY: [u64; 11] = [31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];   // non leap-year
pub const SECONDS_IN_HOUR: u64 = 60 * 60;
pub const SECONDS_IN_YEAR: u64 = 31_536_000;
pub const SECONDS_IN_DAY: u64 = 86_400;

/// A "cycle" is 4 years in days, which includes a leap year
pub const CYCLE_IN_DAYS: u64 = (365 * 3) + 366;
