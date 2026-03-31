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

/// Serde helper: deserialize optional RFC 3339 / ISO 8601 string as `Option<OffsetDateTime>`.
pub mod option_datetime_rfc3339 {
    use super::*;
    use time::format_description::well_known::Rfc3339;

    pub fn serialize<S: Serializer>(dt: &Option<OffsetDateTime>, s: S) -> Result<S::Ok, S::Error> {
        match dt {
            Some(dt) => s.serialize_str(&dt.format(&Rfc3339).map_err(serde::ser::Error::custom)?),
            None => s.serialize_none(),
        }
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(
        d: D,
    ) -> Result<Option<OffsetDateTime>, D::Error> {
        let s = Option::<String>::deserialize(d)?;
        match s {
            Some(s) if !s.is_empty() => OffsetDateTime::parse(&s, &Rfc3339)
                .map(Some)
                .map_err(serde::de::Error::custom),
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

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct OptRfc3339Test {
        #[serde(default, with = "option_datetime_rfc3339")]
        dt: Option<OffsetDateTime>,
    }

    #[test]
    fn timestamp_ms_serde() {
        // round-trip
        let dt = datetime!(2024-01-15 12:30:00 UTC);
        let t = TsTest { ts: dt };
        let json = serde_json::to_string(&t).unwrap();
        let parsed: TsTest = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.ts, dt);

        // from known value
        let parsed: TsTest = serde_json::from_str(r#"{"ts":1705322400000}"#).unwrap();
        assert_eq!(parsed.ts, datetime!(2024-01-15 12:40:00 UTC));

        // direct deserialize
        let mut de = serde_json::Deserializer::from_str("1705322400000");
        let dt = timestamp_ms::deserialize(&mut de).unwrap();
        assert_eq!(dt, datetime!(2024-01-15 12:40:00 UTC));

        // direct serialize
        let dt = datetime!(2024-01-15 12:40:00 UTC);
        let mut buf = Vec::new();
        let mut ser = serde_json::Serializer::new(&mut buf);
        timestamp_ms::serialize(&dt, &mut ser).unwrap();
        assert_eq!(String::from_utf8(buf).unwrap(), "1705322400000");
    }

    #[test]
    fn option_timestamp_ms_serde() {
        // present
        let parsed: OptTsTest = serde_json::from_str(r#"{"ts":1705322400000}"#).unwrap();
        assert_eq!(parsed.ts, Some(datetime!(2024-01-15 12:40:00 UTC)));

        // absent (missing field)
        let parsed: OptTsTest = serde_json::from_str(r#"{}"#).unwrap();
        assert_eq!(parsed.ts, None);

        // serialize None
        let t = OptTsTest { ts: None };
        let json = serde_json::to_string(&t).unwrap();
        assert_eq!(json, r#"{"ts":null}"#);

        // deserialize null
        let parsed: OptTsTest = serde_json::from_str(r#"{"ts":null}"#).unwrap();
        assert_eq!(parsed.ts, None);

        // round-trip Some
        let dt = datetime!(2024-06-15 08:00:00 UTC);
        let t = OptTsTest { ts: Some(dt) };
        let json = serde_json::to_string(&t).unwrap();
        let parsed: OptTsTest = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.ts, Some(dt));

        // direct deserialize
        let mut de = serde_json::Deserializer::from_str("1705322400000");
        let dt = option_timestamp_ms::deserialize(&mut de).unwrap();
        assert_eq!(dt, Some(datetime!(2024-01-15 12:40:00 UTC)));

        // direct serialize
        let dt = Some(datetime!(2024-06-15 08:00:00 UTC));
        let mut buf = Vec::new();
        let mut ser = serde_json::Serializer::new(&mut buf);
        option_timestamp_ms::serialize(&dt, &mut ser).unwrap();
        assert!(!String::from_utf8(buf).unwrap().is_empty());
    }

    #[test]
    fn date_yyyymmdd_serde() {
        // round-trip
        let t = DateTest {
            d: date!(2024 - 01 - 15),
        };
        let json = serde_json::to_string(&t).unwrap();
        assert!(json.contains("20240115"));
        let parsed: DateTest = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.d, date!(2024 - 01 - 15));

        // direct deserialize
        let mut de = serde_json::Deserializer::from_str(r#""20240115""#);
        let d = date_yyyymmdd::deserialize(&mut de).unwrap();
        assert_eq!(d, date!(2024 - 01 - 15));

        // direct serialize
        let d = date!(2024 - 01 - 15);
        let mut buf = Vec::new();
        let mut ser = serde_json::Serializer::new(&mut buf);
        date_yyyymmdd::serialize(&d, &mut ser).unwrap();
        assert!(String::from_utf8(buf).unwrap().contains("20240115"));
    }

    #[test]
    fn option_date_yyyymmdd_serde() {
        // present
        let parsed: OptDateTest = serde_json::from_str(r#"{"d":"20240115"}"#).unwrap();
        assert_eq!(parsed.d, Some(date!(2024 - 01 - 15)));

        // absent
        let parsed: OptDateTest = serde_json::from_str(r#"{}"#).unwrap();
        assert_eq!(parsed.d, None);

        // null
        let parsed: OptDateTest = serde_json::from_str(r#"{"d":null}"#).unwrap();
        assert_eq!(parsed.d, None);

        // empty string
        let parsed: OptDateTest = serde_json::from_str(r#"{"d":""}"#).unwrap();
        assert_eq!(parsed.d, None);

        // serialize None
        let json = serde_json::to_string(&OptDateTest { d: None }).unwrap();
        assert_eq!(json, r#"{"d":null}"#);

        // serialize Some
        let json = serde_json::to_string(&OptDateTest {
            d: Some(date!(2024 - 03 - 15)),
        })
        .unwrap();
        assert!(json.contains("20240315"));

        // round-trip Some
        let t = OptDateTest {
            d: Some(date!(2024 - 06 - 15)),
        };
        let parsed: OptDateTest =
            serde_json::from_str(&serde_json::to_string(&t).unwrap()).unwrap();
        assert_eq!(parsed, t);

        // direct deserialize
        let mut de = serde_json::Deserializer::from_str(r#""20240315""#);
        let d = option_date_yyyymmdd::deserialize(&mut de).unwrap();
        assert_eq!(d, Some(date!(2024 - 03 - 15)));
    }

    #[test]
    fn option_datetime_rfc3339_serde() {
        // present
        let parsed: OptRfc3339Test =
            serde_json::from_str(r#"{"dt":"2016-08-01T00:00:00Z"}"#).unwrap();
        assert_eq!(parsed.dt, Some(datetime!(2016-08-01 00:00:00 UTC)));

        // absent
        let parsed: OptRfc3339Test = serde_json::from_str(r#"{}"#).unwrap();
        assert_eq!(parsed.dt, None);

        // empty string
        let parsed: OptRfc3339Test = serde_json::from_str(r#"{"dt":""}"#).unwrap();
        assert_eq!(parsed.dt, None);

        // serialize None
        let json = serde_json::to_string(&OptRfc3339Test { dt: None }).unwrap();
        assert_eq!(json, r#"{"dt":null}"#);

        // round-trip
        let t = OptRfc3339Test {
            dt: Some(datetime!(2024-01-15 12:30:00 UTC)),
        };
        let json = serde_json::to_string(&t).unwrap();
        assert!(json.contains("2024-01-15T12:30:00Z"));
        let parsed: OptRfc3339Test = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, t);

        // direct deserialize
        let mut de = serde_json::Deserializer::from_str(r#""2024-01-15T12:30:00Z""#);
        let dt = option_datetime_rfc3339::deserialize(&mut de).unwrap();
        assert_eq!(dt, Some(datetime!(2024-01-15 12:30:00 UTC)));

        // direct serialize
        let dt = Some(datetime!(2024-01-15 12:30:00 UTC));
        let mut buf = Vec::new();
        let mut ser = serde_json::Serializer::new(&mut buf);
        option_datetime_rfc3339::serialize(&dt, &mut ser).unwrap();
        assert!(String::from_utf8(buf).unwrap().contains("2024-01-15"));
    }
}
