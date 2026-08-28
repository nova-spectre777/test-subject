package main

import (
    "encoding/json"
    "fmt"
    "os"
)

type Benchmark struct {
    Classification string `json:"classification"`
    Algorithm      string `json:"algorithm"`
    Bits           int    `json:"bits"`
    Seconds        *float64 `json:"seconds,omitempty"`
    Operations     *uint64 `json:"group_operations,omitempty"`
    Verified       bool   `json:"verified"`
}

func main() {
    if len(os.Args) != 2 {
        fmt.Println("usage: aegis-runner <benchmark.json>")
        os.Exit(2)
    }
    data, err := os.ReadFile(os.Args[1])
    if err != nil { panic(err) }
    var b Benchmark
    if err := json.Unmarshal(data, &b); err != nil { panic(err) }
    fmt.Printf("classification=%s algorithm=%s bits=%d verified=%t\n", b.Classification, b.Algorithm, b.Bits, b.Verified)
}
