use std::str::FromStr;

use rust_decimal::Decimal;

pub fn to_u32(b: &[u8]) -> Option<u32> {
    std::str::from_utf8(b).ok()?.trim().parse().ok()
}

pub fn to_u64(b: &[u8]) -> Option<u64> {
    std::str::from_utf8(b).ok()?.trim().parse().ok()
}

pub fn to_usize(b: &[u8]) -> Option<usize> {
    std::str::from_utf8(b).ok()?.trim().parse().ok()
}

pub fn to_str(b: &[u8]) -> Option<String> {
    Some(std::str::from_utf8(b).ok()?.trim().to_string())
}

pub fn to_decimal(b: &[u8]) -> Option<Decimal> {
    Decimal::from_str(std::str::from_utf8(b).ok()?.trim()).ok()
}
