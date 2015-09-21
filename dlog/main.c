#include <stdio.h>
#include <gmp.h>

int main(int argc, char **argv) {
    if (argc != 3) {
        printf("Provide arguments.\n");

        return -1;
    }

    mpz_t a = {};
    mpz_t b = {};
    mpz_init_set_str(a, argv[1], 10);
    mpz_init_set_str(b, argv[2], 10);

    mpz_add(a, a, b);

    printf("%s + %s = %s\n", argv[1], argv[2], mpz_get_str(NULL, 10, a));

    return 0;
}
