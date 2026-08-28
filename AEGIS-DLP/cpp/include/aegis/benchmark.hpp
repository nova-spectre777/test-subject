#pragma once
#include <cstdint>
#include <string>

namespace aegis {
struct BenchmarkResult {
    std::string algorithm;
    std::uint64_t operations{};
    bool verified{};
};
}
