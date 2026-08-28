use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DlpInstance {
    pub group_bits: u32,
    pub generator: u64,
    pub target: u64,
    pub order: u64,
    pub secret_bound: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructureReport {
    pub order_bits: u32,
    pub interval_known: bool,
    pub interval_width: Option<u64>,
    pub subgroup_score: f64,
    pub distinguished_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plan {
    pub algorithm: String,
    pub estimated_operations: f64,
    pub reason: String,
}

pub fn probe(x: &DlpInstance) -> StructureReport {
    let order_bits = 64 - x.order.leading_zeros();
    let width = x.secret_bound;
    let interval_known = width.is_some();
    let interval_width = width;
    let subgroup_score = if x.order > 0 && x.group_bits > 0 { 0.5 } else { 0.0 };
    let distinguished_score = 0.0;
    StructureReport { order_bits, interval_known, interval_width, subgroup_score, distinguished_score }
}

pub fn plan(r: &StructureReport) -> Plan {
    if let Some(width) = r.interval_width {
        let cost = (width as f64).sqrt().max(1.0);
        return Plan { algorithm: "kangaroo-toy".into(), estimated_operations: cost, reason: "A bounded interval is explicitly known; use interval-specific cost model.".into() };
    }
    let rho = 2f64.powf((r.order_bits as f64) / 2.0);
    Plan { algorithm: "pollard-rho-toy".into(), estimated_operations: rho, reason: "No exploitable interval was declared; use the generic square-root baseline.".into() }
}

pub fn certificate_hash(instance: &DlpInstance, report: &StructureReport, plan: &Plan, verified: bool) -> String {
    let record = serde_json::json!({"instance":instance,"structure":report,"plan":plan,"verified":verified});
    let mut h = Sha256::new();
    h.update(serde_json::to_vec(&record).expect("serializable"));
    format!("sha256:{:x}", h.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn interval_structure_selects_interval_engine() {
        let x = DlpInstance { group_bits: 20, generator: 2, target: 1, order: 1048575, secret_bound: Some(256) };
        let p = probe(&x);
        assert_eq!(plan(&p).algorithm, "kangaroo-toy");
    }
}
