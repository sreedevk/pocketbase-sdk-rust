use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Deserializer};

pub fn parse(raw: &str) -> Result<DateTime<Utc>, chrono::ParseError> {
    let trimmed = raw.trim_end_matches('Z');
    NaiveDateTime::parse_from_str(trimmed, "%Y-%m-%d %H:%M:%S%.f").map(|naive| naive.and_utc())
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let raw = String::deserialize(deserializer)?;
    parse(&raw).map_err(serde::de::Error::custom)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_timestamp_with_trailing_z() {
        let dt = parse("2022-06-22 07:13:00.643Z").unwrap();
        assert_eq!(dt.to_rfc3339(), "2022-06-22T07:13:00.643+00:00");
    }

    #[test]
    fn parses_timestamp_without_trailing_z() {
        let dt = parse("2022-06-25 11:03:35.163").unwrap();
        assert_eq!(dt.to_rfc3339(), "2022-06-25T11:03:35.163+00:00");
    }

    #[test]
    fn parses_timestamp_without_fraction() {
        let dt = parse("2022-06-25 11:03:35Z").unwrap();
        assert_eq!(dt.to_rfc3339(), "2022-06-25T11:03:35+00:00");
    }

    #[test]
    fn parses_timestamp_without_z_or_fraction() {
        let dt = parse("2022-06-25 11:03:35").unwrap();
        assert_eq!(dt.to_rfc3339(), "2022-06-25T11:03:35+00:00");
    }
}
