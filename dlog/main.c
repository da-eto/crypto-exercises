#include <stdio.h>
#include <stdlib.h>
#include <gmp.h>
#include "include/benchmark.h"

typedef struct num {
    int pow;
    mpz_t n;
} num;

int cmp_num(const void *a, const void *b) {
    return mpz_cmp(((num *) a)->n, ((num *) b)->n);
}

int main(int argc, char **argv) {
    if (argc != 5) {
        printf("Provide arguments.\n");

        return -1;
    }

    unsigned long long start = benchmark_us(0);

    mpz_t p = {};
    mpz_t g = {};
    mpz_t h = {};
    mpz_init_set_str(p, argv[1], 10);
    mpz_init_set_str(g, argv[2], 10);
    mpz_init_set_str(h, argv[3], 10);
    int base = 1 << (atoi(argv[4]) / 2);

    const int power_limit = base + 1;
    num *h_g = malloc(sizeof(num) * power_limit);

    mpz_t g_inv = {};
    mpz_invert(g_inv, g, p);
    h_g[0].pow = 0;
    mpz_init_set(h_g[0].n, h);

    for (int i = 1; i < power_limit; ++i) {
        h_g[i].pow = i;
        mpz_init(h_g[i].n);
        mpz_mul(h_g[i].n, h_g[i - 1].n, g_inv);
        mpz_mod(h_g[i].n, h_g[i].n, p);
    }

    qsort(h_g, (size_t) power_limit, sizeof(num), cmp_num);

    mpz_t g_b = {};
    mpz_init(g_b);
    mpz_powm_ui(g_b, g, (unsigned long) base, p);

    num g_m = {};
    mpz_init_set_ui(g_m.n, 1ul);

    for (g_m.pow = 0; g_m.pow < power_limit; ++g_m.pow, mpz_mul(g_m.n, g_m.n, g_b), mpz_mod(g_m.n, g_m.n, p)) {
        num *found = (num *) bsearch(&g_m, h_g, (size_t) power_limit, sizeof(num), cmp_num);

        if (found != NULL) {
            printf("Found in %llu us\n", benchmark_us(start));
            long long pow = (long long) (found->pow) + (long long) g_m.pow * (long long) base;
            printf("Found! x1 = %d, x0 = %d, pow = x1 + x0*base = %lld, log_%s(%s) = %lld mod %s\n",
                   found->pow, g_m.pow, pow, mpz_get_str(NULL, 10, g), mpz_get_str(NULL, 10, h), pow,
                   mpz_get_str(NULL, 10, p));

            break;
        }
    }

    printf("All in %llu us\n", benchmark_us(start));

    return 0;
}
