// From musl src/ctype/punct.h
// Licensed under the MIT license
// Copyright 2005-2020 Rich Felker, et al.



pub fn is(wc: usize) -> ::c_uchar {
    if wc < 0x20000 {
        return (table[(table[wc >> 8] as usize) * 32 + ((wc & 255) >> 3)] >> (wc & 7)) & 1;
    }
    return 0;
}

const table: [::c_uchar; 4000] = [
    18, 16, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 16, 16, 34, 35, 16, 36, 37,
    38, 39, 40, 41, 42, 43, 16, 44, 45, 46, 17, 17, 47, 17, 17, 17, 17, 17, 17, 48, 49, 50, 51, 52,
    53, 54, 55, 17, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16,
    16, 16, 16, 16, 16, 56, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16,
    16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16,
    16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16,
    16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 57, 16, 58, 59,
    60, 61, 62, 63, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16,
    16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16,
    16, 16, 16, 16, 16, 16, 16, 16, 64, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16,
    16, 16, 16, 16, 16, 16, 16, 16, 65, 16, 16, 66, 16, 67, 68, 69, 16, 70, 71, 72, 16, 73, 16, 16,
    74, 75, 76, 77, 78, 16, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 16, 92, 93, 94, 95,
    16, 16, 16, 16, 96, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 97, 16, 16, 16,
    16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16,
    16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16,
    16, 16, 98, 99, 16, 16, 100, 101, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16,
    16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16,
    16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16,
    16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 102, 16, 16, 16, 16, 16, 16, 16, 16, 16,
    16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 103, 104, 105, 106, 16, 16, 107, 108, 17, 17, 109, 16,
    16, 16, 16, 16, 16, 110, 111, 16, 16, 16, 16, 16, 112, 113, 16, 16, 114, 115, 116, 16, 117,
    118, 119, 17, 17, 17, 120, 121, 122, 123, 124, 16, 16, 16, 16, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0, 254, 255, 0, 252, 1, 0, 0, 248, 1, 0, 0,
    120, 0, 0, 0, 0, 255, 251, 223, 251, 0, 0, 128, 0, 0, 0, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 60, 0, 252, 255, 224, 175, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 223, 255, 255, 255, 255, 255, 32, 64, 176, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 252, 3, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 252, 0, 0, 0, 0, 0, 230, 254, 255, 255,
    255, 0, 64, 73, 0, 0, 0, 0, 0, 24, 0, 255, 255, 0, 216, 0, 0, 0, 0, 0, 0, 0, 1, 0, 60, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 224, 1, 30, 0, 96, 255, 191, 0, 0, 0, 0, 0, 0, 255, 7, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 248, 207, 227, 0, 0, 0, 3, 0, 32, 255, 127, 0,
    0, 0, 78, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 7, 252, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    16, 0, 32, 30, 0, 48, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 16, 0, 32, 0, 0, 0, 0, 252, 111, 0, 0, 0,
    0, 0, 0, 0, 16, 0, 32, 0, 0, 0, 0, 64, 0, 0, 0, 0, 0, 0, 0, 0, 16, 0, 32, 0, 0, 0, 0, 3, 224,
    0, 0, 0, 0, 0, 0, 0, 16, 0, 32, 0, 0, 0, 0, 253, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 0, 0, 0, 0,
    255, 7, 16, 0, 0, 0, 0, 0, 0, 0, 0, 32, 0, 0, 0, 0, 128, 255, 16, 0, 0, 0, 0, 0, 0, 16, 0, 32,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 0, 160, 0, 127, 0, 0, 255, 3, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 4, 0, 0, 0, 0, 16, 0, 0, 0, 0, 0, 0, 128, 0, 128, 192, 223, 0, 12, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 4, 0, 31, 0, 0, 0, 0, 0, 0, 254, 255, 255, 255, 0, 252, 255, 255, 0, 0, 0, 0, 0, 0,
    0, 0, 252, 0, 0, 0, 0, 0, 0, 192, 255, 223, 255, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 6, 0,
    252, 0, 0, 0, 0, 0, 0, 0, 0, 0, 192, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 224, 255, 255, 255, 31, 0, 0, 255, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 96, 0, 0, 1, 0, 0, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 56, 0, 0, 0, 0,
    16, 0, 0, 0, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 48, 0, 0, 254, 127, 47, 0, 0,
    255, 3, 255, 127, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 49, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    196, 255, 255, 255, 255, 0, 0, 0, 192, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 224, 159, 0, 0, 0, 0, 127,
    63, 255, 127, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 0, 16, 0, 0, 252, 255, 255, 255,
    31, 0, 0, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 64, 0, 12, 240, 0, 0, 0, 0, 0, 0, 128, 248, 0, 0, 0,
    0, 0, 0, 0, 192, 0, 0, 0, 0, 0, 0, 0, 0, 255, 0, 255, 255, 255, 33, 144, 3, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 127, 0, 224, 251, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 160, 3, 224, 0, 224, 0, 224,
    0, 96, 128, 248, 255, 255, 255, 252, 255, 255, 255, 255, 255, 127, 223, 255, 241, 127, 255,
    127, 0, 0, 255, 255, 255, 255, 0, 0, 255, 255, 255, 255, 1, 0, 123, 3, 208, 193, 175, 66, 0,
    12, 31, 188, 255, 255, 0, 0, 0, 0, 0, 14, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 127, 0, 0, 0, 255, 7, 0, 0, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 63, 0, 0, 0, 0, 0, 0, 252, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 207, 255, 255, 255, 63, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 224, 135, 3, 254, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 128, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 255, 127, 255, 255, 255, 255,
    0, 0, 0, 0, 0, 0, 255, 255, 255, 251, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 15, 0,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 63, 0, 0, 0, 255, 15, 30, 255, 255, 255, 1, 252, 193, 224,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 0, 0, 0, 0, 255, 255, 255, 255, 15, 0, 0, 0, 255,
    255, 255, 127, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 255, 255, 127, 0, 0, 0, 0, 0, 0, 192,
    0, 224, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 15, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 255, 0, 255, 255, 127, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 64, 0, 0, 0, 0, 15, 255, 3, 0, 0, 0, 0, 0, 0, 240, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    16, 192, 0, 0, 255, 255, 3, 23, 0, 0, 0, 0, 0, 248, 0, 0, 0, 0, 8, 128, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 8, 0, 255, 63, 0, 192, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 240, 0, 0, 128, 3, 0,
    0, 0, 0, 0, 0, 0, 128, 2, 0, 0, 192, 0, 0, 67, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 56, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 252, 255, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 192, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 48, 255, 255, 255, 3, 255, 255,
    255, 255, 255, 255, 247, 255, 127, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128,
    254, 255, 0, 252, 1, 0, 0, 248, 1, 0, 0, 248, 63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    127, 127, 0, 48, 135, 255, 255, 255, 255, 255, 143, 255, 0, 0, 0, 0, 0, 0, 224, 255, 255, 127,
    255, 15, 1, 0, 0, 0, 0, 0, 255, 255, 255, 255, 255, 63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 255, 15, 0, 0, 0, 0, 15, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 128, 255, 0, 0, 128, 255, 0, 0, 0, 0, 128, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 248, 0, 0,
    192, 143, 0, 0, 0, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 48, 255, 255, 252, 255,
    255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 135, 255, 1, 255, 1, 0, 0, 0, 224, 0, 0, 0, 224, 0, 0,
    0, 0, 0, 1, 0, 0, 96, 248, 127, 0, 0, 0, 0, 0, 0, 0, 0, 254, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0,
    0, 30, 0, 254, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 252, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255,
    255, 127, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 224, 127, 0, 0, 0, 192, 255,
    255, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    192, 63, 252, 255, 63, 0, 0, 128, 3, 0, 0, 0, 0, 0, 0, 254, 3, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 24, 0, 15, 0, 0, 0, 0, 0, 56, 0, 0, 0, 0, 0, 0, 0, 0, 0, 225, 63, 0, 232, 254, 255,
    31, 0, 0, 0, 0, 0, 0, 0, 96, 63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0,
    6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 0, 32, 0, 0, 192, 31, 31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 68, 248, 0, 104, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 76,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128,
    255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 14, 0, 0, 0, 255, 31, 0, 0, 0, 0, 0, 0,
    0, 0, 192, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 252, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 252, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 128, 255, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 223, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 62, 0, 0,
    252, 255, 31, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 52,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 1, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 255, 255, 3, 128, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 255, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 192, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 63,
    0, 0, 0, 0, 0, 0, 0, 255, 255, 48, 0, 0, 248, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 255, 7, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 176, 15, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 63, 0, 255, 255,
    255, 255, 127, 254, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 1, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255, 63, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 15, 0, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 127, 0, 255, 255, 255, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 8, 0, 0, 0,
    8, 0, 0, 32, 0, 0, 0, 32, 0, 0, 128, 0, 0, 0, 128, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 8, 0, 0, 0, 0,
    0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    15, 0, 248, 254, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 127, 0, 0, 128, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 240, 0, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 255, 127, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    112, 7, 0, 192, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 254, 255, 255, 255, 255, 255, 255, 255, 31, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    254, 255, 255, 255, 255, 255, 255, 63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 3, 0, 255, 255, 255, 255, 255, 15, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 15, 0, 255, 127, 254, 255, 254, 255, 254, 255, 255, 255, 63, 0, 255, 31, 255, 255,
    255, 255, 0, 0, 0, 252, 0, 0, 0, 28, 0, 0, 0, 252, 255, 255, 255, 31, 0, 0, 0, 0, 0, 0, 192,
    255, 255, 255, 7, 0, 255, 255, 255, 255, 255, 15, 255, 1, 3, 0, 63, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 63, 0, 255, 31, 255, 7, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 15, 0, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 1, 255, 15, 0, 0, 255, 15, 255, 255, 255, 255, 255, 255,
    255, 0, 255, 3, 255, 255, 255, 255, 255, 0, 255, 255, 255, 63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    255, 239, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 123, 252, 255, 255, 255,
    255, 231, 199, 255, 255, 255, 231, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 15, 0, 255, 63, 15, 7, 7, 0, 63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];
