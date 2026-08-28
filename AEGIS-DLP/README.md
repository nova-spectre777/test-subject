# AEGIS-DLP

**Adaptive Elliptic-Group Search**

A reproducible research harness for testing whether structure-aware adaptive planning can outperform generic discrete-log baselines on structured instances.

## Research policy

AEGIS-DLP keeps three result classes strictly separate:

- **Measured** — actually executed and independently verified.
- **Extrapolated** — derived from measured data/models and explicitly labeled.
- **Theoretical** — analytical/asymptotic estimates, never presented as completed computation.

There will be no fabricated 150-bit results and no claim that an extrapolated workload was completed.

## Architecture

```text
DLP instance -> Structure probes -> Adaptive planner
                    |                 /  |  \
                    |              BSGS Rho Kangaroo
                    +-------------------+---+
                                        |
                                   Verification
                                        |
                                    Certificate
                                        |
                                     Benchmark
```

## Multi-language design

- **Rust** — arbitrary-precision-safe core types, structure measurements, planning, verification and certificate primitives. The core is not restricted to `u64`.
- **C++20** — performance-oriented algorithm engines and group-operation instrumentation.
- **Go** — experiment orchestration and reproducible benchmark execution.
- **Python** — offline statistics, result analysis and visualization.

## Algorithm roadmap

1. Baby-step giant-step for medium-size measured instances.
2. Pollard rho with distinguished-point instrumentation.
3. Pollard kangaroo for explicitly bounded intervals.
4. Adaptive structure probing and cost-based algorithm selection.
5. Larger-size methods only when genuinely implemented and independently benchmarked.
6. Index-calculus/NFS-style research is optional and will not be listed as implemented until it is real, tested, and benchmarked.

## Benchmark tiers

| Size | Evidence policy |
|---|---|
| 10–40 bit | Exhaustive measured validation |
| 40–80 bit | Measured algorithm comparison |
| 80–150 bit | Measured only when actually completed; otherwise extrapolated |
| 150+ bit | Theoretical/extrapolated unless independently completed |

Every benchmark should record hardware, compiler, implementation version, parameters, random seed where applicable, wall time, memory, group-operation counts, and verification status.

## External comparison

Where licensing and availability permit, results may be compared against SageMath, Magma and CADO-NFS or other established reference implementations. Comparisons must state versions, hardware, parameters and whether results are measured or estimated.

## Certificate pipeline

```text
INSTANCE -> STRUCTURE MEASUREMENTS -> ALGORITHM SELECTION
        -> GROUP OPERATIONS -> INDEPENDENT VERIFICATION
        -> BENCHMARK RECORD -> CERTIFICATE
```

A certificate is valid only when an independent verifier confirms the returned exponent against the original group instance.

## Demonstrable advantage

The primary research question is whether the adaptive controller can identify useful structure early enough to improve performance versus generic baselines on the same hardware. Candidate structure includes known intervals, subgroup/order information and other measurable instance properties. Negative results are first-class research results.

## Practical packaging

The project is designed for simple build/test commands and an optional Docker environment. Users should be able to feed a documented finite-group instance to the harness and obtain either a verified result or an explicit classification explaining whether the workload is measured, extrapolated, or theoretical.

## Scope and current claim

**Near-term claim:** a reproducible adaptive controller for structured DLP instances, with measured evidence for when structure-aware selection improves on generic baselines.

This project does **not** claim to break generic hard 150-bit discrete logarithms. A genuine 100–160-bit measured result will be reported only when the computation is actually completed and independently verified.
