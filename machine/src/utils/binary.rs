pub trait Split<T> {
    fn split(self) -> T;
}

pub trait Combine<T> {
    fn combine(parts: T) -> Self;
}

// ========== u16 ==========

impl Split<(u8, u8)> for u16 {
    fn split(self) -> (u8, u8) {
        let h: u8 = (self >> 8).try_into().unwrap();
        let l: u8 = (self & 0x00ff).try_into().unwrap();
        (h, l)
    }
}

impl Combine<(u8, u8)> for u16 {
    fn combine(parts: (u8, u8)) -> Self {
        ((parts.0 as u16) << 8) | parts.1 as u16
    }
}

// ========== u32 ==========

impl Split<(u16, u16)> for u32 {
    fn split(self) -> (u16, u16) {
        let h: u16 = (self >> 16).try_into().unwrap();
        let l: u16 = (self & 0x0000ffff).try_into().unwrap();
        (h, l)
    }
}

impl Combine<(u16, u16)> for u32 {
    fn combine(parts: (u16, u16)) -> Self {
        ((parts.0 as u32) << 16) | parts.1 as u32
    }
}

impl Split<(u8, u8, u8, u8)> for u32 {
    fn split(self) -> (u8, u8, u8, u8) {
        let val: (u16, u16) = u32::split(self);
        let (h1, h2): (u8, u8) = u16::split(val.0);
        let (l1, l2): (u8, u8) = u16::split(val.1);
        (h1, h2, l1, l2)
    }
}

impl Combine<(u8, u8, u8, u8)> for u32 {
    fn combine(parts: (u8, u8, u8, u8)) -> Self {
        let h = u16::combine((parts.0, parts.1));
        let l = u16::combine((parts.2, parts.3));
        u32::combine((h, l))
    }
}

// ========== u64 ==========

impl Split<(u32, u32)> for u64 {
    fn split(self) -> (u32, u32) {
        let h: u32 = (self >> 32).try_into().unwrap();
        let l: u32 = (self & 0x00000000ffffffff).try_into().unwrap();
        (h, l)
    }
}

impl Combine<(u32, u32)> for u64 {
    fn combine(parts: (u32, u32)) -> Self {
        ((parts.0 as u64) << 32) | parts.1 as u64
    }
}

impl Split<(u16, u16, u16, u16)> for u64 {
    fn split(self) -> (u16, u16, u16, u16) {
        let val: (u32, u32) = u64::split(self);
        let (h1, h2): (u16, u16) = u32::split(val.0);
        let (l1, l2): (u16, u16) = u32::split(val.1);
        (h1, h2, l1, l2)
    }
}

impl Combine<(u16, u16, u16, u16)> for u64 {
    fn combine(parts: (u16, u16, u16, u16)) -> Self {
        let h = u32::combine((parts.0, parts.1));
        let l = u32::combine((parts.2, parts.3));
        u64::combine((h, l))
    }
}

impl Split<(u8, u8, u8, u8, u8, u8, u8, u8)> for u64 {
    fn split(self) -> (u8, u8, u8, u8, u8, u8, u8, u8) {
        let val: (u16, u16, u16, u16) = u64::split(self);
        let a = u16::split(val.0);
        let b = u16::split(val.1);
        let c = u16::split(val.2);
        let d = u16::split(val.3);
        (a.0, a.1, b.0, b.1, c.0, c.1, d.0, d.1)
    }
}

impl Combine<(u8, u8, u8, u8, u8, u8, u8, u8)> for u64 {
    fn combine(parts: (u8, u8, u8, u8, u8, u8, u8, u8)) -> Self {
        let h = u32::combine((parts.0, parts.1, parts.2, parts.3));
        let l = u32::combine((parts.4, parts.5, parts.6, parts.7));
        u64::combine((h, l))
    }
}