/*
 * Assuming compilation from ffi/ directory:
 * 
 * cargo build --release
 * cc -o tdprf -std=c99 -pedantic -Wall -Werror examples/tdprf.c -Isrc/ -L../../target/release/ -lsocioty_ffi
 * ./tdprf
 */

#include <stdio.h>
#include <stdint.h>
#include <string.h>
#include <sys/random.h>

#include <socioty_ffi.h>

void u8_array_println(uint8_t *x, size_t len)
{
    printf("[");
    for (size_t i = 0; i < len; i++)
    {
        printf("%" PRIu8, x[i]);
        if (i != len - 1)
        {
            printf(", ");
        }
    }
    printf("]\n");
}

int main(void)
{
    uint8_t *input = (uint8_t *)"55228022"; // 1656840660 / 30

    const char *clients[5];
    clients[0] = "localhost:5680";
    clients[1] = "localhost:5681";
    clients[2] = "localhost:5682";
    clients[3] = "localhost:5683";
    clients[4] = "localhost:5684";

    uint8_t *output_keys[5];
    tdprf_generate(clients, 5, 3, output_keys);

    uint8_t *partial_evals[5];
    for (int i = 0; i < 5; i++) {
        partial_evals[i] = tdprf_partial_eval(output_keys[i], input, strlen((const char *)input));
    }

    uint8_t *self_key = tdprf_random_key();

    printf("tdPRF Output (k = 5 >= 3): ");
    uint8_t *reconstructed = tdprf_reconstruct(self_key, input, strlen((const char *)input), clients, 5, (const uint8_t **) partial_evals, 5);
    u8_array_println(reconstructed, 32);
    tdprf_free(reconstructed);

    printf("tdPRF Output (k = 4 >= 3): ");
    reconstructed = tdprf_reconstruct(self_key, input, strlen((const char *)input), clients, 4, (const uint8_t **) partial_evals, 4);
    u8_array_println(reconstructed, 32);
    tdprf_free(reconstructed);

    printf("tdPRF Output (k = 3 >= 3): ");
    reconstructed = tdprf_reconstruct(self_key, input, strlen((const char *)input), clients, 3, (const uint8_t **) partial_evals, 3);
    u8_array_println(reconstructed, 32);
    tdprf_free(reconstructed);

    printf("tdPRF Output (k = 2 < 3): ");
    reconstructed = tdprf_reconstruct(self_key, input, strlen((const char *)input), clients, 2, (const uint8_t **) partial_evals, 2);
    u8_array_println(reconstructed, 32);
    tdprf_free(reconstructed);

    tdprf_free(self_key);

    for (int i = 0; i < 5; i++) {
        tdprf_free(partial_evals[i]);
        tdprf_free(output_keys[i]);
    }
}
