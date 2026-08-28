//! AEGIS-DLP core types.
//! The research core intentionally uses arbitrary-precision integers rather than u64.

use num_bigint::BigUint;
use num_traits::Zero;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DlpInstance {
    pub modulus: String,
    pub generator: String,
    pub residue: String,
    pub order: Option<String>,
    pub lower_bound: Option<String>,
    pub upper_bound: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructureMeasurements {
    pub modulus_bits: usize,
    pub order_bits: Option<usize>,
    pub interval_width_bits: Option<usize>,
    pub has_known_interval: bool,
    pub has_known_order: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanDecision {
    pub algorithm: String,
    pub rationale: String,
    pub estimated_cost: String,
}

pub fn parse_hex(value: &str) -> Result<BigUint, String> {
    BigUint::parse_bytes(value.trim_start_matches("0x").as_bytes(), 16)
        .ok_or_else(|| format!("invalid hexadecimal integer: {value}"))
}

pub fn measure(instance: &DlpInstance) -> Result<StructureMeasurements, String> {
    let p = parse_hex(&instance.modulus)?;
    let order = instance.order.as_deref().map(parse_hex).transpose()?;
    let interval_width_bits = match (&instance.lower_bound, &instance.upper_bound) {
        (Some(lo), Some(hi)) => {
            let lo = parse_hex(lo)?;
            let hi = parse_hex(hi)?;
            if hi < lo { return Err("upper bound is smaller than lower bound".into()); }
            let width = hi - lo;
            Some(if width.is_zero() { 0 } else { width.bits() as usize })
        }
        _ => None,
    };
    Ok(StructureMeasurements {
        modulus_bits: p.bits() as usize,
        order_bits: order.map(|x| x.bits() as usize),
        interval_width_bits,
        has_known_interval: instance.lower_bound.is_some() && instance.upper_bound.is_some(),
        has_known_order: instance.order.is_some(),
    })
}

/// Stable digest used to bind a certificate to its exact instance and measurements.
pub fn certificate_digest(instance: &DlpInstance, measurements: &StructureMeasurements, plan: &PlanDecision) -> String {
    let payload = serde_json::to_vec(&(instance, measurements, plan)).expect("serializable certificate payload");
    let digest = Sha256::digest(payload);
    format!("sha256:{digest:x}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn supports_large_integers() {
        let n = parse_hex("ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc5").unwrap();
        assert!(n.bits() > 200);
    }
}
