use chrono::{NaiveDate, NaiveDateTime};

pub fn parse_date(date: &str) -> Option<NaiveDate> {
    match NaiveDate::parse_from_str(date, "%Y-%m-%d") {
        Ok(res) => Some(res),
        Err(_) => match NaiveDate::parse_from_str(date, "%Y/%m/%d") {
            Ok(res) => Some(res),
            Err(_) => None,
        },
    }
}

pub fn format_datetime(datetime: NaiveDateTime) -> String {
    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn format_date(datetime: NaiveDate) -> String {
    datetime.format("%Y-%m-%d").to_string()
}
