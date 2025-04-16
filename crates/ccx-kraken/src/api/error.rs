use std::fmt::Display;

use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, derive_more::Error)]
pub struct KrakenApiError(#[error(ignore)] Vec<String>);

impl Display for KrakenApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Got errors from Kraken API:")?;
        for error in &self.0 {
            writeln!(f, "\t- {error}")?;
        }

        Ok(())
    }
}
