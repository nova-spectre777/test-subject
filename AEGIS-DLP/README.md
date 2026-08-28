# AEGIS-DLP

**Adaptive Elliptic-Group Search**

AEGIS-DLP is a research framework for studying whether structure-aware adaptive search can outperform generic discrete-logarithm methods on deliberately structured, reproducible instances.

## Research question

> Can an adaptive controller identify exploitable structure before committing to an expensive generic DLP algorithm?

AEGIS does not claim to break generic 150-bit discrete logarithms. Results are explicitly separated into measured computation, analytical extrapolation, and theoretical estimates.

## Architecture

```text
DLP instance
     |
Structure probe
     |
+----+------------------+
|       |               |
interval distinguished subgroup
structure behavior    structure
|       |               |
+-------+---------------+
        |
Adaptive planner
        |
Search engine
        |
Verification
        |
Certificate
```

## Multi-language design

- **Rust** — core arithmetic, instance model, structure probes, planner interfaces, verification and certificate primitives.
- **C++** — high-performance experimental search engines and low-level benchmarking.
- **Go** — experiment orchestration, reproducible benchmark execution and future distributed runners.
- **Python** — statistical analysis, plotting and research notebooks.

## Benchmark policy

| Range | Purpose |
|---|---|
| 10–40 bit | Exhaustive validation |
| 40–80 bit | Algorithm benchmarking |
| 80–150 bit | Analytical/extrapolated benchmark |
| 150 bit | Theoretical target |

No fabricated 150-bit result and no presentation of an extrapolated workload as a completed computation.

## Certificate principle

Every claimed improvement should be traceable through:

```text
INSTANCE
  -> STRUCTURE MEASUREMENTS
  -> ALGORITHM SELECTION
  -> GROUP OPERATIONS
  -> VERIFICATION
  -> BENCHMARK
  -> CERTIFICATE
```

Certificates should record the instance parameters, probe measurements, selected strategy, operation counts, verification outcome, timing/environment metadata, and benchmark classification.

## Research status

Early-stage research scaffold. Performance claims require independent reproduction and statistical validation.
