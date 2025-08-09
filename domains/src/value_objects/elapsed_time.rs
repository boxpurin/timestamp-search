use errors::AppResult;
use types::impl_numeric_value;
use std::str::FromStr;
use tracing_subscriber::fmt::format;
use errors::AppError::DomainParseError;

impl_numeric_value!(ElapsedTime, u64);

impl ElapsedTime {
    pub fn new(seconds: u64) -> AppResult<Self> {
        Ok(ElapsedTime(seconds))
    }

    pub fn from_hhmmss(s : &str) -> AppResult<Self> {
        {
            let parts: Vec<&str> = s.split(':').collect();

            match parts.len() {
                2 => {
                    let minutes = parts[0]
                        .parse::<u64>()
                        .map_err(|e| DomainParseError(e.to_string()))?;
                    let seconds = parts[1]
                        .parse::<u64>()
                        .map_err(|e| DomainParseError(e.to_string()))?;
                    Self::new(minutes * 60 + seconds)
                }
                3 => {
                    let hours = parts[0]
                        .parse::<u64>()
                        .map_err(|e| DomainParseError(e.to_string()))?;
                    let minutes = parts[1]
                        .parse::<u64>()
                        .map_err(|e| DomainParseError(e.to_string()))?;
                    let seconds = parts[2]
                        .parse::<u64>()
                        .map_err(|e| DomainParseError(e.to_string()))?;
                    Self::new(hours * 60 * 60 + minutes * 60 + seconds)
                }
                _ => Err(
                    DomainParseError(format!("from ElapsedTime : Invalid time format : {}", s))
                ),
            }
        }
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn valid_seconds() {
        assert!(ElapsedTime::new(0).is_ok());
        assert!(ElapsedTime::new(1000).is_ok());
        assert!(ElapsedTime::new(1000000).is_ok());
    }

    #[rstest::rstest]
    #[test]
    #[case("2:12:23", 2*60*60+12*60+23)]
    #[case("12:23", 12*60+23)]
    #[case("01:23", 1*60+23)]
    #[case("0:23", 23)]
    fn valid_hhmmss(#[case] fmt: &str, #[case] expected: u64) {
        let e = ElapsedTime::from_hhmmss(fmt);
        assert!(e.is_ok());
        let e = e.unwrap();
        assert_eq!(expected, e);
    }

    #[rstest::rstest]
    #[test]
    #[case(":12:23")]
    #[case("23")]
    fn invalid_hhmmss(#[case] fmt: &str) {
        assert!(ElapsedTime::from_hhmmss(fmt).is_err());
    }

}
