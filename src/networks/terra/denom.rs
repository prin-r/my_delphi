//! Exchange rate denominations

use crate::error::Error;
use crate::sources::band::*;
use std::fmt::{self, Display};
use stdtx::Decimal;

/// Denomination
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Denom {
    /// Korean  Wan
    UKRW,

    /// Singaporean Dollar
    UMNT,

    /// IMF Special Drawing Rights
    USDR,

    /// US Dollars
    UUSD,
}

impl Denom {
    /// Get a slice of the [`Denom`] kinds
    pub fn kinds() -> &'static [Denom] {
        &[Denom::UKRW, Denom::UMNT, Denom::USDR, Denom::UUSD]
    }

    /// Get the code corresponding to a [`Denom`]
    pub fn as_str(self) -> &'static str {
        match self {
            Denom::UKRW => "ukrw",
            Denom::UMNT => "umnt",
            Denom::USDR => "usdr",
            Denom::UUSD => "uusd",
        }
    }

    /// Get the exchange rate for this [`Denom`]
    pub async fn get_exchange_rate(self) -> Result<Decimal, Error> {
        // TODO(tarcieri): compute non-abstain votes for each denom
        let x = BandSource::new(1, "0000000442414e4400000000000f4240".into(), 4, 4);
        match x.request_data().await {
            Ok(y) => Ok(Decimal::from(y)),
            Err(_z) => Ok(Decimal::from(-1i8)),
        }
        // Ok(Decimal::from(-1i8))
    }
}

impl Display for Denom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
