"""Offline benchmark analysis.

Only consumes recorded benchmark data; it never labels extrapolation as measurement.
"""
import json
import sys
from collections import Counter

ALLOWED = {"measured", "extrapolated", "theoretical"}


def load(path):
    with open(path, encoding="utf-8") as f:
        rows = json.load(f)
    if not isinstance(rows, list):
        raise ValueError("benchmark file must contain a JSON list")
    for row in rows:
        if row.get("classification") not in ALLOWED:
            raise ValueError("unknown benchmark classification")
    return rows


def main(path):
    rows = load(path)
    print("records:", len(rows))
    print("classification:", dict(Counter(r["classification"] for r in rows)))
    measured = [r for r in rows if r["classification"] == "measured"]
    verified = [r for r in measured if r.get("verified") is True]
    print("measured:", len(measured), "verified:", len(verified))


if __name__ == "__main__":
    if len(sys.argv) != 2:
        raise SystemExit("usage: python analyze.py benchmarks.json")
    main(sys.argv[1])
