use std::str::FromStr;

use anyhow::Result;
use rust_decimal::Decimal;

pub fn to_u32(b: &[u8]) -> Result<u32> {
    Ok(std::str::from_utf8(b)?.trim().parse()?)
}

pub fn to_u64(b: &[u8]) -> Result<u64> {
    Ok(std::str::from_utf8(b)?.trim().parse()?)
}

pub fn to_usize(b: &[u8]) -> Result<usize> {
    Ok(std::str::from_utf8(b)?.trim().parse()?)
}

pub fn to_str(b: &[u8]) -> Result<String> {
    Ok(std::str::from_utf8(b)?.trim().to_string())
}

pub fn to_decimal(b: &[u8]) -> Result<Decimal> {
    Ok(Decimal::from_str(std::str::from_utf8(b)?.trim())?)
}
