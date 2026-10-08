use std::num::NonZeroU64;

use super::{Error, Result};

#[derive(Debug)]
pub struct ChargeRequest {
    reference: String,
    amount_minor: NonZeroU64,
}

impl ChargeRequest {
    pub fn new(reference: &str, amount_minor: u64) -> Result<Self> {
        if reference.trim().is_empty() {
            return Err(Error::InvalidReference);
        }
        let amount_minor = NonZeroU64::new(amount_minor).ok_or(Error::InvalidAmount)?;
        Ok(Self {
            reference: reference.to_owned(),
            amount_minor,
        })
    }

    pub fn reference(&self) -> &str {
        &self.reference
    }

    pub fn amount_minor(&self) -> u64 {
        self.amount_minor.get()
    }
}
