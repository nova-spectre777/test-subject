package main

import (
    "encoding/json"
    "fmt"
    "math"
    "os"
)

type Record struct {
    Bits int `json:"bits"`
    Class string `json:"benchmark_class"`
    GenericEstimate float64 `json:"generic_sqrt_estimate"`
    Note string `json:"note"`
}

func main() {
    bits := 32
    if len(os.Args) > 1 { fmt.Sscanf(os.Args[1], "%d", &bits) }
    r := Record{Bits: bits, Class: classify(bits), GenericEstimate: math.Pow(2, float64(bits)/2), Note: "Estimate only; measured results must be supplied separately."}
    out, _ := json.MarshalIndent(r, "", "  ")
    fmt.Println(string(out))
}

func classify(bits int) string {
    switch {
    case bits <= 40: return "exhaustive-validation"
    case bits <= 80: return "measured-benchmark"
    case bits <= 150: return "analytical-extrapolation"
    default: return "out-of-scope"
    }
}
