//! AEGIS-DLP core research primitives.
//!
//! This crate intentionally provides instance modeling and adaptive-planning
//! interfaces rather than claiming a shortcut for generic discrete logs.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Strategy {
    Exhaustive,
    Bsgs,
    PollardRho,
    PollardKangaroo,
    Analytical,
}

#[derive(Debug, Clone)]
pub struct InstanceProfile {
    pub bit_size: u32,
    pub interval_hint: bool,
    pub subgroup_hint: bool,
    pub distinguished_behavior: bool,
}

#[derive(Debug, Clone)]
pub struct Plan {
    pub strategy: Strategy,
    pub reason: &'static str,
}

/// Conservative baseline planner used by the research harness.
pub fn choose_strategy(p: &InstanceProfile) -> Plan {
    if p.bit_size <= 40 {
        return Plan { strategy: Strategy::Exhaustive, reason: "small instance; exhaustive validation" };
    }
    if p.interval_hint {
        return Plan { strategy: Strategy::PollardKangaroo, reason: "measured interval structure" };
    }
    if p.subgroup_hint {
        return Plan { strategy: Strategy::Bsgs, reason: "measured subgroup structure" };
    }
    if p.distinguished_behavior {
        return Plan { strategy: Strategy::PollardRho, reason: "measured distinguished-point behavior" };
    }
    if p.bit_size > 80 {
        return Plan { strategy: Strategy::Analytical, reason: "large target; extrapolation only" };
    }
    Plan { strategy: Strategy::PollardRho, reason: "generic baseline" }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_instances_are_exhaustively_validated() {
        let p = InstanceProfile { bit_size: 32, interval_hint: false, subgroup_hint: false, distinguished_behavior: false };
        assert_eq!(choose_strategy(&p).strategy, Strategy::Exhaustive);
    }
}
