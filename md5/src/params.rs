pub struct MD5Params {
    pub a: u32,
    pub b: u32,
    pub c: u32,
    pub d: u32,
    pub x: u32,
    pub s: u8,
    pub ac: u32,
}

impl MD5Params {
    pub fn new(a: u32, b: u32, c: u32, d: u32, x: u32, s: u8, ac: u32) -> Self {
        Self {
            a,
            b,
            c,
            d,
            x,
            s,
            ac,
        }
    }
}

pub fn f(params: &mut MD5Params) -> u32 {
    // let a = params.a;
    let b = params.b;
    let c = params.c;
    let d = params.d;
    // let x = params.x;
    // let s = params.s;
    // let ac = params.ac;

    let res = md5_f(b, c, d);

    params.a = params
        .a
        .wrapping_add(res.wrapping_add(params.x.wrapping_add(params.ac)));
    params.a = rotate_left(params.a, params.s.into());
    params.a = params.a.wrapping_add(params.b);

    b.wrapping_add(res)
}

pub fn g(params: &mut MD5Params) -> u32 {
    // let a = params.a;
    let b = params.b;
    let c = params.c;
    let d = params.d;
    // let x = params.x;
    // let s = params.s;
    // let ac = params.ac;

    params.a = params.a.wrapping_add(params.x.wrapping_add(params.ac));
    params.a = params.a.wrapping_add(!d & c);
    params.a = params.a.wrapping_add(d & b);
    params.a = rotate_left(params.a, params.s.into());
    params.a = params.a.wrapping_add(params.b);

    b.wrapping_add(!d & c)
}

pub fn h(params: &mut MD5Params) -> u32 {
    // let a = params.a;
    let b = params.b;
    let c = params.c;
    let d = params.d;
    // let x = params.x;
    // let s = params.s;
    // let ac = params.ac;

    params.a = params.a.wrapping_add(params.x.wrapping_add(params.ac));
    params.a = params.a.wrapping_add(b ^ c ^ d);
    params.a = rotate_left(params.a, params.s.into());
    params.a = params.a.wrapping_add(params.b);

    b.wrapping_add(b ^ c ^ d)
}

pub fn i(params: &mut MD5Params) -> u32 {
    // let a = params.a;
    let b = params.b;
    let c = params.c;
    let d = params.d;
    // let x = params.x;
    // let s = params.s;
    // let ac = params.ac;

    params.a = params.a.wrapping_add(params.x.wrapping_add(params.ac));
    params.a = params.a.wrapping_add(c ^ (d | !b));
    params.a = rotate_left(params.a, params.s.into());
    params.a = params.a.wrapping_add(params.b);

    b.wrapping_add(c ^ (d | !b))
}
#[inline(always)]
pub fn md5_f(x: u32, y: u32, z: u32) -> u32 {
    (x & y) | (!x & z)
}

#[inline(always)]
pub fn md5_h(x: u32, y: u32, z: u32) -> u32 {
    x ^ y ^ z
}

#[inline(always)]
pub fn md5_i(x: u32, y: u32, z: u32) -> u32 {
    y ^ (x | !z)
}

#[inline(always)]
pub fn rotate_left(x: u32, n: u32) -> u32 {
    (x << n) | (x >> (32 - n))
}
