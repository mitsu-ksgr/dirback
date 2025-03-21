//!
//! # Timestamp
//!
//! Timestamp is wrapper for the `TimeDate<Utc>`.
//!
//! Support the `YYYYMMDDThhmmssZ` format.
//!

use anyhow::Context;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Timestamp(DateTime<Utc>);

impl Timestamp {
    pub fn now() -> Self {
        Self(Utc::now())
    }

    pub fn from_utc(ts: &DateTime<Utc>) -> Self {
        Self(*ts)
    }

    /// Parses a `YYYYMMDDThhmmssZ` format string into a Timestamp value.
    pub fn from_fmt_str(fmt: &str) -> anyhow::Result<Self> {
        // "Z" can't be parsed with "%z", it is converted to "+00:00".
        let str_ts = &fmt.replace("Z", "+00:00");
        let ts = DateTime::parse_from_str(str_ts, "%Y%m%dT%H%M%S%z")
            .with_context(|| format!("Failed to parse timestamp from {}", fmt))?;
        Ok(Self(ts.to_utc()))
    }

    /// Returns formatted timestamp
    pub fn fmt(&self) -> String {
        self.format("%Y%m%dT%H%M%SZ").to_string()
    }
}

impl std::ops::Deref for Timestamp {
    type Target = DateTime<Utc>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_from_utc() {
        let now = Utc::now();
        let ts = Timestamp::from_utc(&now);
        assert_eq!(ts.0, now);
    }

    #[test]
    fn test_new_from_fmt_str() {
        let fmt_str = "20250123T123456Z";
        let ts = Timestamp::from_fmt_str(fmt_str);
        assert!(ts.is_ok());

        let ts = ts.unwrap();
        assert_eq!(ts.to_string(), "2025-01-23 12:34:56 UTC");
        assert_eq!(ts.fmt(), fmt_str);
    }

    #[test]
    fn test_new_from_fmt_str_error() {
        let dt_str = "2025-01-23T12:34:56Z";
        let ts = Timestamp::from_fmt_str(dt_str);
        assert!(ts.is_err());

        let err = ts.unwrap_err();
        assert!(err.to_string().contains(dt_str));
    }

    #[test]
    fn test_fmt() {
        let dt_str = "2025-01-23T12:34:56Z";
        let dt = DateTime::parse_from_rfc3339(dt_str).unwrap().to_utc();
        let ts = Timestamp::from_utc(&dt);
        assert_eq!(ts.fmt(), "20250123T123456Z")
    }

    #[test]
    fn it_serializable() {
        let src = Timestamp::now();

        let s = serde_json::to_string(&src);
        assert!(s.is_ok(), "it should be serializable into json.");

        let dst = serde_json::from_str(&s.unwrap());
        assert!(dst.is_ok(), "it should be deserializable from json.");

        let dst: Timestamp = dst.unwrap();
        assert_eq!(dst, src);
    }
}
