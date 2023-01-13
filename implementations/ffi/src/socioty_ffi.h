#ifndef SOCIOTY_FFI_H
#define SOCIOTY_FFI_H

#include <inttypes.h>

void tdprf_generate(const char **clients, size_t clients_len, uint32_t k, uint8_t **output_keys);
uint8_t *tdprf_partial_eval(const uint8_t *key, const uint8_t *input, size_t input_len);
uint8_t *tdprf_reconstruct(const uint8_t *key, const uint8_t *input, size_t input_len, const char **clients, size_t clients_len, const uint8_t **partials, size_t partials_len);
uint8_t *tdprf_random_key(void);
uint8_t *tdprf_free(uint8_t *x);

#endif
