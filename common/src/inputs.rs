pub const EMPTY: &'static [u8] = b"";

pub const MF_HUGE: &'static [u8] =
    include_bytes!("./data/mini-functions-huge.txt");
pub const MF_SMALL: &'static [u8] =
    include_bytes!("./data/mini-functions-small.txt");
pub const MF_TINY: &'static [u8] =
    include_bytes!("./data/mini-functions-tiny.txt");
