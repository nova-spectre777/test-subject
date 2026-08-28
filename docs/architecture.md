# AEGIS-DLP architecture

## Controller loop

1. Parse a controlled DLP instance.
2. Run deterministic structure probes.
3. Produce a feature vector with explicit confidence and evidence.
4. Score only mathematically justified candidate algorithms.
5. Select the lowest-cost eligible strategy subject to benchmark limits.
6. Execute and count group operations.
7. Verify the result independently.
8. Hash the complete record into a certificate.

## Structure probes

### Interval

An explicitly supplied bound can make interval-specific methods appropriate. The planner must record the bound rather than infer a secret from unsupported heuristics.

### Subgroup/order

Factorization or known order information can expose smaller search domains. The probe records what is known and how it affects the cost model.

### Distinguished behavior

For parallel collision-style experiments, the framework records distinguished-point parameters and observed rates. This is a performance measurement, not evidence of a shortcut.

## Adaptive policy

The controller is deterministic for a fixed instance, configuration, and software version. Every decision includes a reason string and estimated cost. A future learned policy may be evaluated as a separate research subject, but it cannot silently replace the mathematical planner.

## Reproducibility

A certificate includes the instance description, structure measurements, planner decision, operation count, verification status, implementation version, and SHA-256 digest. Benchmark results are never mixed with extrapolated values.
