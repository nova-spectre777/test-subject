#include <cstdint>
#include <iostream>
#include <string>
#include <cmath>

// Benchmark-only cost model. This intentionally does not claim to solve
// large real-world discrete-log instances.
struct Estimate { std::string name; double operations; };

Estimate rho(uint32_t bits) { return {"pollard-rho", std::pow(2.0, bits / 2.0)}; }
Estimate bsgs(uint32_t bits) { return {"bsgs", std::pow(2.0, bits / 2.0)}; }
Estimate kangaroo(uint64_t interval) { return {"kangaroo", std::sqrt(static_cast<double>(interval))}; }

int main() {
    const uint32_t bits = 32;
    auto a = rho(bits);
    auto b = bsgs(bits);
    auto c = kangaroo(1ULL << 16);
    std::cout << a.name << " " << a.operations << "\n";
    std::cout << b.name << " " << b.operations << "\n";
    std::cout << c.name << " " << c.operations << "\n";
    return 0;
}
