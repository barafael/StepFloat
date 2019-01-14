#include <math.h>

#include <stdint.h>
#include <stdio.h>

/*
 * Type punning - allowed in C, not in C++ (but works on gcc)
 */
typedef union {
    float content;
    /* bitfields */
    struct {
        uint32_t mantissa : 23;
        uint32_t exponent :  8;
        uint32_t sign     :  1;
    } parts;
} float_parts;

/* Increase a float by exactly one tick. */
static void increase_by_tick(float *to_increase) {
    float_parts fp = { .content = *to_increase };
    if (fp.parts.mantissa == 0b11111111111111111111111) {
        fp.parts.exponent += 1;
        fp.parts.mantissa = 0;
    } else {
        fp.parts.mantissa += 1;
    }
    *to_increase = fp.content;
}

static float last = 0.0;

uint64_t num_ticks_from(float a, float b) {
    uint64_t counter = 0;

    if (a > b) {
        float swap_tmp = a;
        a = b;
        b = swap_tmp;
    }

    last = a;

    static float sort_of_eps = 0.00000001;

    for (float runner = a; runner <= b; increase_by_tick(&runner)) {
        counter++;
        if ((last - runner) > sort_of_eps || (runner - last) > sort_of_eps) {
            last = runner;
            printf("%.12f\n", runner);
        }
    }
    return counter;
}

int main() {
    float start = 0.0f;
    float end   = M_PI/1024.0;
    printf("Between %f and %f, there are %lu distinct float values.\n", start, end, num_ticks_from(start, end));
}
