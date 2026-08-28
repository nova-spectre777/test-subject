# AEGIS-DLP

**Adaptive Elliptic-Group Search**

AEGIS-DLP is a reproducible research framework for testing whether **structure-aware adaptive search** can outperform a generic discrete-log baseline on deliberately structured, controlled instances.

## Research question

> Can an adaptive controller identify exploitable structure before committing to an expensive generic DLP algorithm?

AEGIS probes an instance, estimates the cost and applicability of candidate strategies, selects a justified strategy, executes it on a bounded benchmark, verifies the result, and emits a reproducible certificate.

```text
DLP instance
     |
     v
Structure probe
     |
     +--> interval structure
     +--> subgroup/order structure
     +--> distinguished-point behavior
     |
     v
Adaptive planner
     |
     v
Search engine
     |
     v
Verification --> Certificate --> Benchmark
```

## Multi-language architecture

- **Rust** — safe high-performance core, instance model, probes, planner interfaces and certificates.
- **C++20** — optimized experimental search engines and operation-count benchmarks.
- **Go** — benchmark orchestration and reproducible experiment runner.
- **Python** — offline analysis, statistics and visualization.

The languages communicate through simple JSON/JSONL experiment records rather than hidden state.

## Benchmark policy

| Size | Purpose | Status |
|---|---|---|
| 10–40 bit | exhaustive validation | measured |
| 40–80 bit | algorithm comparison | measured where practical |
| 80–150 bit | cost-model benchmarking | analytical/extrapolated |
| 150 bit | research target | theoretical only |

AEGIS-DLP **never fabricates a 150-bit result** and never labels an extrapolation as a completed computation.

## Certificate-first rule

Every claimed improvement should be traceable as:

`INSTANCE -> STRUCTURE MEASUREMENTS -> ALGORITHM SELECTION -> OPERATIONS -> VERIFICATION -> BENCHMARK -> CERTIFICATE`

No AI prediction is treated as a cryptographic result.

## Scope

The initial implementation is intentionally restricted to **toy/controlled finite groups and benchmark instances**. It is a research harness, not a wallet-key recovery tool or a production cryptanalysis service.

## Build

See `docs/architecture.md` and the language-specific READMEs under `core/rust`, `engines/cpp`, `runner/go`, and `analysis/python`.

## License

Apache-2.0. See `LICENSE`.
