// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use jiff::{
    ToSpan,
    civil::{Date, Weekday},
};

/// Gets the first date that will be visible on the calendar
pub fn get_calendar_first(year: i16, month: i8, from_weekday: Weekday) -> Date {
    let date = Date::new(year, month, 1).expect("valid date");
    let num_days = date.weekday().since(from_weekday);
    date.checked_sub(num_days.days()).expect("valid date")
}

/// Gets the ISO 8601 week number of a date
pub fn get_week_number(date: Date) -> i8 {
    date.iso_week_date().week()
}

#[cfg(test)]
mod tests {
    use super::*;
    use jiff::civil::date;

    #[test]
    fn iso_week_number() {
        assert_eq!(get_week_number(date(2026, 10, 26)), 44);
        // Jan 1st 2027 is a Friday and belongs to the last week of 2026
        assert_eq!(get_week_number(date(2027, 1, 1)), 53);
        // Dec 29th 2025 is a Monday and starts week 1 of 2026
        assert_eq!(get_week_number(date(2025, 12, 29)), 1);
    }
}
