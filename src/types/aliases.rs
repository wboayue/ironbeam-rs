use serde::{self, Deserialize, Deserializer, Serializer};
use time::{Date, OffsetDateTime};

/// Account identifier (max 10 chars).
pub type AccountId = String;

/// Exchange-qualified symbol, e.g. `"XCME:ES.U16"`.
pub type Symbol = String;

/// Price value.
pub type Price = f64;

/// Quantity value.
pub type Quantity = f64;

/// Volume value.
pub type Volume = f64;

/// Numeric string, e.g. `"123.456"`.
pub type NumberString = String;

/// ISO 4217 currency code, e.g. `"USD"`.
pub type CurrencyCode = String;

/// Order identifier (max 50 chars).
pub type OrderId = String;

/// Strategy identifier.
pub type StrategyId = i64;

/// Order update identifier.
pub type OrderUpdateId = String;

/// Position identifier (max 50 chars).
pub type PositionId = String;

/// Stream session identifier (UUID).
pub type StreamSessionId = String;

/// Indicator identifier (max 100 chars).
pub type IndicatorId = String;

/// Sub-account identifier (max 10 chars).
pub type SubAccountId = String;

/// Spread definition string.
pub type Spread = String;

/// Market complex type (max 50 chars).
pub type MarketComplexType = String;

/// Serde helper: deserialize i64 (ms since epoch) as `OffsetDateTime`.
pub mod timestamp_ms {
    use super::*;

    pub fn serialize<S: Serializer>(dt: &OffsetDateTime, s: S) -> Result<S::Ok, S::Error> {
        let ms = (dt.unix_timestamp_nanos() / 1_000_000) as i64;
        s.serialize_i64(ms)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<OffsetDateTime, D::Error> {
        let ms = i64::deserialize(d)?;
        OffsetDateTime::from_unix_timestamp_nanos(ms as i128 * 1_000_000)
            .map_err(serde::de::Error::custom)
    }
}

/// Serde helper: deserialize optional i64 (ms since epoch) as `Option<OffsetDateTime>`.
pub mod option_timestamp_ms {
    use super::*;

    pub fn serialize<S: Serializer>(dt: &Option<OffsetDateTime>, s: S) -> Result<S::Ok, S::Error> {
        match dt {
            Some(dt) => timestamp_ms::serialize(dt, s),
            None => s.serialize_none(),
        }
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(
        d: D,
    ) -> Result<Option<OffsetDateTime>, D::Error> {
        let ms = Option::<i64>::deserialize(d)?;
        match ms {
            Some(ms) => OffsetDateTime::from_unix_timestamp_nanos(ms as i128 * 1_000_000)
                .map(Some)
                .map_err(serde::de::Error::custom),
            None => Ok(None),
        }
    }
}

/// Serde helper: deserialize `"YYYYMMDD"` string as `time::Date`.
pub mod date_yyyymmdd {
    use super::*;
    use time::macros::format_description;

    pub fn serialize<S: Serializer>(date: &Date, s: S) -> Result<S::Ok, S::Error> {
        let fmt = format_description!("[year][month][day]");
        s.serialize_str(&date.format(fmt).map_err(serde::ser::Error::custom)?)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Date, D::Error> {
        let s = String::deserialize(d)?;
        let fmt = format_description!("[year][month][day]");
        Date::parse(&s, fmt).map_err(serde::de::Error::custom)
    }
}

/// Serde helper: deserialize optional `"YYYYMMDD"` string as `Option<time::Date>`.
pub mod option_date_yyyymmdd {
    use super::*;

    pub fn serialize<S: Serializer>(date: &Option<Date>, s: S) -> Result<S::Ok, S::Error> {
        match date {
            Some(date) => date_yyyymmdd::serialize(date, s),
            None => s.serialize_none(),
        }
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Option<Date>, D::Error> {
        let s = Option::<String>::deserialize(d)?;
        match s {
            Some(s) if !s.is_empty() => {
                let fmt = time::macros::format_description!("[year][month][day]");
                Date::parse(&s, fmt)
                    .map(Some)
                    .map_err(serde::de::Error::custom)
            }
            _ => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};
    use time::macros::{date, datetime};

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct TsTest {
        #[serde(with = "timestamp_ms")]
        ts: OffsetDateTime,
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct OptTsTest {
        #[serde(default, with = "option_timestamp_ms")]
        ts: Option<OffsetDateTime>,
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct DateTest {
        #[serde(with = "date_yyyymmdd")]
        d: Date,
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct OptDateTest {
        #[serde(default, with = "option_date_yyyymmdd")]
        d: Option<Date>,
    }

    #[test]
    fn timestamp_ms_round_trip() {
        let dt = datetime!(2024-01-15 12:30:00 UTC);
        let t = TsTest { ts: dt };
        let json = serde_json::to_string(&t).unwrap();
        let parsed: TsTest = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.ts, dt);
    }

    #[test]
    fn timestamp_ms_from_known_value() {
        let parsed: TsTest = serde_json::from_str(r#"{"ts":1705322400000}"#).unwrap();
        assert_eq!(parsed.ts, datetime!(2024-01-15 12:40:00 UTC));
    }

    #[test]
    fn option_timestamp_ms_present() {
        let parsed: OptTsTest = serde_json::from_str(r#"{"ts":1705322400000}"#).unwrap();
        assert_eq!(parsed.ts, Some(datetime!(2024-01-15 12:40:00 UTC)));
    }

    #[test]
    fn option_timestamp_ms_absent() {
        let parsed: OptTsTest = serde_json::from_str(r#"{}"#).unwrap();
        assert_eq!(parsed.ts, None);
    }

    #[test]
    fn date_yyyymmdd_round_trip() {
        let t = DateTest {
            d: date!(2024 - 01 - 15),
        };
        let json = serde_json::to_string(&t).unwrap();
        assert!(json.contains("20240115"));
        let parsed: DateTest = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.d, date!(2024 - 01 - 15));
    }

    #[test]
    fn option_date_absent() {
        let parsed: OptDateTest = serde_json::from_str(r#"{}"#).unwrap();
        assert_eq!(parsed.d, None);
    }

    #[test]
    fn option_date_present() {
        let parsed: OptDateTest = serde_json::from_str(r#"{"d":"20240115"}"#).unwrap();
        assert_eq!(parsed.d, Some(date!(2024 - 01 - 15)));
    }
}
