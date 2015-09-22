#include <iostream>
#include <map>
#include <gmpxx.h>
#include "include/benchmark.h"

//namespace std {
//    template<>
//    struct hash<mpz_class> {
//        std::size_t operator()(const mpz_class &k) const {
//            std::size_t h = 37;
//            mpz_srcptr src = k.get_mpz_t();
//
//            for (int i = 0; i < abs(src->_mp_size) && i < 10; ++i) {
//                h ^= h + h * static_cast<std::size_t>(src->_mp_d[i]);
//            }
//
//            return h;
//        }
//    };
//}
//
int main(int argc, char **argv) {
    if (argc != 5) {
        printf("Provide arguments.\n");

        return -1;
    }

    unsigned long long start = benchmark_us(0);
    mpz_class p(argv[1], 10);
    mpz_class g(argv[2], 10);
    mpz_class h(argv[3], 10);
    unsigned long base = 1UL << (atoi(argv[4]) / 2);

    const unsigned long power_limit = base + 1;
    std::map<mpz_class, unsigned long> hG = {};

    mpz_class gInv;
    mpz_invert(gInv.get_mpz_t(), g.get_mpz_t(), p.get_mpz_t());

    {
        mpz_class hPowG(h);
        hG.emplace(hPowG, 0);

        for (unsigned long i = 1; i < power_limit; ++i) {
            hPowG *= gInv;
            hPowG %= p;
            hG.emplace(hPowG, i);
        }
    }

    mpz_class gB;
    mpz_powm_ui(gB.get_mpz_t(), g.get_mpz_t(), base, p.get_mpz_t());
    mpz_class gM = 1;

    for (unsigned long power = 0; power < power_limit; ++power, gM *= gB) {
        auto found = hG.find(gM);

        if (found != hG.end()) {
            std::cout << "Found in " << benchmark_us(start) << " us" << std::endl;
            unsigned long long pow = (unsigned long long) (found->second) +
                                     (unsigned long long) power * (unsigned long long) base;
            std::cout << "Found! x1 = " << found->second << ", x0 = " << power << ", pow = x1 + x0*base = " << pow;
            std::cout << ", log_" << g << "(" << h << ") = " << pow << " mod " << p << std::endl;

            break;
        }
    }

    std::cout << "Time: " << benchmark_us(start) << " us." << std::endl;

    return 0;
}
