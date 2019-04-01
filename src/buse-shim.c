#include <argp.h>
#include <err.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "buse.h"

#define UNUSED(x) (void)(x)

static int (*_read)(void *, u_int32_t, u_int64_t, void *);

static void *_read_ctx;

static int (*_write)(const void *, u_int32_t, u_int64_t, void *);

static void *_write_ctx;

static int xmp_read(void *buf, u_int32_t len, u_int64_t offset, void *userdata) {
    //memcpy(buf, (char *)data + offset, len);
    UNUSED(userdata);

    return _read(buf, len, offset, _read_ctx);
}

static int xmp_write(const void *buf, u_int32_t len, u_int64_t offset, void *userdata) {
    //memcpy((char *)data + offset, buf, len);
    UNUSED(userdata);

    return _write(buf, len, offset, _write_ctx);
}

static void xmp_disc(void *userdata) {
    UNUSED(userdata);
}

static int xmp_flush(void *userdata) {
    UNUSED(userdata);
    return 0;
}

static int xmp_trim(u_int64_t from, u_int32_t len, void *userdata) {
    UNUSED(from);
    UNUSED(len);
    UNUSED(userdata);
    return 0;
}

int buse_main_shim(const char *device, u_int64_t size,
                   int (*read)(void *, u_int32_t, u_int64_t, void *), void *read_ctx,
                   int (*write)(const void *, u_int32_t, u_int64_t, void *), void *write_ctx) {
    _read = read;
    _read_ctx = read_ctx;
    _write = write;
    _write_ctx = write_ctx;

    struct buse_operations aop = {
            .read = xmp_read,
            .write = xmp_write,
            .disc = xmp_disc,
            .flush = xmp_flush,
            .trim = xmp_trim,
            .size = size,
    };

    return buse_main(device, &aop, NULL);
}