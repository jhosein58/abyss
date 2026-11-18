#include <stdint.h>
#include <stdlib.h>

typedef struct {
    int8_t status;
    double total;
} NativeResult;

extern int abyss_generated_main(int, char **);
extern NativeResult AbyssCustomExport_42(double, int32_t);
extern void abyss_update(int32_t *, int32_t);
extern int32_t abyss_only_called_from_c(int32_t);
extern int32_t abyss_local_export(int32_t, int32_t);

int main(int argc, char **argv) {
    NativeResult value = AbyssCustomExport_42(1.25, -7);
    int32_t updated = 0;
    abyss_update(&updated, -5);
    if (value.status != -7 || value.total != -4.5 || updated != -5 || abyss_local_export(-10, 4) != 5 ||
        abyss_only_called_from_c(12) != 10)
        abort();
    return abyss_generated_main(argc, argv);
}
