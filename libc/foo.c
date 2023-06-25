#include <stdint.h>
#include <stddef.h>

uint8_t foo(uint8_t a, uint8_t b)
{
  return a + b;
}

typedef struct
{
  uint8_t const *ptr; // non-NULL
  size_t len;
} slice_ref_uint8_t;

void print_buf(uint8_t *ptr, size_t len)
{
  ptr[1] = 99;
  ptr[3] = 230;
}
