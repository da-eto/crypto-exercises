#ifndef DLOG_BENCHMARK_H
#define DLOG_BENCHMARK_H

#include <stddef.h>
#include <sys/time.h>
#include "benchmark.h"

inline unsigned long long benchmark_us(unsigned long long start) {
    struct timeval tm_current;
    gettimeofday(&tm_current, NULL);

    return 1000000UL * tm_current.tv_sec + tm_current.tv_usec - start;
}

#endif //DLOG_BENCHMARK_H
