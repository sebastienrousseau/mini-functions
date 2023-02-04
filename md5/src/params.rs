/// MD5Params
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MD5Params {
    /// A
    pub a: u32,
    /// B
    pub b: u32,
    /// C
    pub c: u32,
    /// D
    pub d: u32,
    /// X
    pub x: u32,
    /// S
    pub s: u8,
    /// AC
    pub ac: u32,
}

impl MD5Params {
    /// Create a new instance of the MD5Params struct.
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
/// Rotate left
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
/// Rotate left
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
/// Rotate left
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
/// Rotate left
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
/// md5_f
pub fn md5_f(x: u32, y: u32, z: u32) -> u32 {
    (x & y) | (!x & z)
}

#[inline(always)]
/// md5_h
pub fn md5_h(x: u32, y: u32, z: u32) -> u32 {
    x ^ y ^ z
}
/// md5_i
#[inline(always)]
pub fn md5_i(x: u32, y: u32, z: u32) -> u32 {
    y ^ (x | !z)
}
/// Rotate left
#[inline(always)]
pub fn rotate_left(x: u32, n: u32) -> u32 {
    (x << n) | (x >> (32 - n))
}
