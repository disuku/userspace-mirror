/*
 * busexmp - example memory-based block device using BUSE
 * Copyright (C) 2013 Adam Cozzette
 *
 * This program is free software; you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation; either version 2 of the License, or
 *  (at your option) any later version.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License along
 *  with this program; if not, write to the Free Software Foundation, Inc.,
 *  51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.
 */

#include <argp.h>
#include <err.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "buse.h"

static FILE *file;

static int xmp_read(void *buf, u_int32_t len, u_int64_t offset, void *userdata) {
    //memcpy(buf, (char *)data + offset, len);
    printf("xmp_read\n");

    fseek(file, offset, SEEK_SET);
    fread(buf, len, 1, file);

    return 0;
}

static int xmp_write(const void *buf, u_int32_t len, u_int64_t offset, void *userdata) {
    //memcpy((char *)data + offset, buf, len);
    printf("xmp_write\n");

    fseek(file, offset, SEEK_SET);
    fwrite(buf, len, 1, file);

    return 0;
}

static void xmp_disc(void *userdata) {
}

static int xmp_flush(void *userdata) {
    return 0;
}

static int xmp_trim(u_int64_t from, u_int32_t len, void *userdata) {
    return 0;
}


int main(int argc, char *argv[]) {
    for (int i = 0; i < argc; i++) {
        printf("arg: %s\n", argv[i]);
    }

    char *device = argv[1];
    printf("device: %s\n", device);
    char *src_device = argv[2];
    printf("src_device: %s\n", src_device);

    file = fopen(src_device, "r+");
    if (file == NULL) return 1;
    fseek(file, 0L, SEEK_END);
    unsigned long size = (unsigned long) ftell(file);

    struct buse_operations aop = {
            .read = xmp_read,
            .write = xmp_write,
            .disc = xmp_disc,
            .flush = xmp_flush,
            .trim = xmp_trim,
            .size = size,
    };

    printf("main\n");

    int status = buse_main(device, &aop, NULL);

    fclose(file);
    return status;
}
