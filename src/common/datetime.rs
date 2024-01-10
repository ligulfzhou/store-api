use chrono::{NaiveDate, NaiveDateTime};
use regex::Regex;

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

pub fn parse_date_with_regex(date: &str) -> Option<NaiveDate> {
    let dash_re = Regex::new(r"(\d{4}-\d{1,2}-\d{1,2})").unwrap();
    let mut matched = vec![];
    for (_, [ymd]) in dash_re.captures_iter(date).map(|c| c.extract()) {
        matched.push(ymd);
    }

    if !matched.is_empty() {
        if let Ok(naive_date) = NaiveDate::parse_from_str(matched[0], "%Y-%m-%d") {
            return Some(naive_date);
        }
    }

    let slash_re = Regex::new(r"(\d{4}/\d{1,2}/\d{1,2})").unwrap();
    let mut matched = vec![];
    for (_, [ymd]) in slash_re.captures_iter(date).map(|c| c.extract()) {
        matched.push(ymd);
    }

    if !matched.is_empty() {
        if let Ok(naive_date) = NaiveDate::parse_from_str(matched[0], "%Y/%m/%d") {
            return Some(naive_date);
        }
    }

    None
}
