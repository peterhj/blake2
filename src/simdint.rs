// Copyright 2015 blake2-rfc Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#![allow(dead_code)]

use crate::simdty::{Simd4};

#[cfg(feature = "simd")]
extern "platform-intrinsic" {
    pub fn simd_add<T>(x: T, y: T) -> T;
    pub fn simd_shl<T>(x: T, y: T) -> T;
    pub fn simd_shr<T>(x: T, y: T) -> T;
    pub fn simd_xor<T>(x: T, y: T) -> T;

    pub fn simd_shuffle2<T, U>(v: T, w: T, idx: [u32; 2]) -> U;
    pub fn simd_shuffle4<T, U>(v: T, w: T, idx: [u32; 4]) -> U;
    pub fn simd_shuffle8<T, U>(v: T, w: T, idx: [u32; 8]) -> U;
    pub fn simd_shuffle16<T, U>(v: T, w: T, idx: [u32; 16]) -> U;
    pub fn simd_shuffle32<T, U>(v: T, w: T, idx: [u32; 32]) -> U;
}

/* __shuffle_vector4 derived from packed_simd:

Copyright (c) 2014 The Rust Project Developers

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE. */

#[inline]
pub unsafe fn __shuffle_vector4<const IDX: [u32; 4], T>(x: Simd4<T>, y: Simd4<T>) -> Simd4<T> {
    simd_shuffle4(x, y, IDX)
}
