"""Offline analysis for AEGIS-DLP JSONL benchmark records."""
import json
import math
import sys


def classify(bits: int) -> str:
    if bits <= 40:
        return "exhaustive-validation"
    if bits <= 80:
        return "measured-benchmark"
    if bits <= 150:
        return "analytical-extrapolation"
    return "out-of-scope"


def main(path: str) -> None:
    rows = [json.loads(line) for line in open(path, encoding="utf-8") if line.strip()]
    for r in rows:
        bits = int(r["bits"])
        baseline = 2 ** (bits / 2)
        r["benchmark_class"] = classify(bits)
        r["baseline_sqrt_estimate"] = baseline
    print(json.dumps(rows, indent=2, sort_keys=True))


if __name__ == "__main__":
    if len(sys.argv) != 2:
        raise SystemExit("usage: python analyze.py results.jsonl")
    main(sys.argv[1])
