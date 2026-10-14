use std::ops::Mul;
use gl::types::GLfloat;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Col8 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Col32 {
    pub r: u32,
    pub g: u32,
    pub b: u32,
    pub a: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Col64 {
    pub r: u64,
    pub g: u64,
    pub b: u64,
    pub a: u64,
}

// Col8
impl Col8 {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self::new_a(r, g, b, 255)
    }

    pub const fn new_a(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub fn pack(&self) -> u32 {
        ((self.r as u32) << 24)
            | ((self.g as u32) << 16)
            | ((self.b as u32) << 8)
            | (self.a as u32)
    }

    pub fn as_vals(&self) -> (GLfloat,GLfloat,GLfloat,GLfloat){
        ((self.r / 255) as GLfloat, (self.g /255)  as GLfloat,( self.b)  as GLfloat, (self.a) as GLfloat)
    }

}

// Col32
impl Col32 {
    pub fn new(r: u32, g: u32, b: u32) -> Self {
        Self::new_a(r, g, b, u32::MAX)
    }

    pub fn new_a(r: u32, g: u32, b: u32, a: u32) -> Self {
        Self { r, g, b, a }
    }

    pub fn pack(&self) -> u128 {
        ((self.r as u128) << 96)
            | ((self.g as u128) << 64)
            | ((self.b as u128) << 32)
            | (self.a as u128)
    }
}

// Col64
impl Col64 {
    pub const fn new(r: u64, g: u64, b: u64) -> Self {
        Self::new_a(r, g, b, u64::MAX)
    }

    pub const fn new_a(r: u64, g: u64, b: u64, a: u64) -> Self {
        Self { r, g, b, a }
    }
}

// Conversion: Col8 -> Col32
impl From<Col8> for Col32 {
    fn from(c: Col8) -> Self {
        Self {
            r: c.r as u32,
            g: c.g as u32,
            b: c.b as u32,
            a: c.a as u32,
        }
    }
}

// Conversion: Col32 -> Col64
impl From<Col32> for Col64 {
    fn from(c: Col32) -> Self {
        Self {
            r: c.r as u64,
            g: c.g as u64,
            b: c.b as u64,
            a: c.a as u64,
        }
    }
}

// Conversion: Col32 -> Col8 (may truncate)
impl From<Col32> for Col8 {
    fn from(c: Col32) -> Self {
        Self {
            r: c.r as u8,
            g: c.g as u8,
            b: c.b as u8,
            a: c.a as u8,
        }
    }
}

// Conversion: Col64 -> Col32 (may truncate)
impl From<Col64> for Col32 {
    fn from(c: Col64) -> Self {
        Self {
            r: c.r as u32,
            g: c.g as u32,
            b: c.b as u32,
            a: c.a as u32,
        }
    }
}

// Conversion: Col8 -> Col64
impl From<Col8> for Col64 {
    fn from(c: Col8) -> Self {
        Self {
            r: c.r as u64,
            g: c.g as u64,
            b: c.b as u64,
            a: c.a as u64,
        }
    }
}

// Conversion: Col64 -> Col8 (may truncate)
impl From<Col64> for Col8 {
    fn from(c: Col64) -> Self {
        Self {
            r: c.r as u8,
            g: c.g as u8,
            b: c.b as u8,
            a: c.a as u8,
        }
    }
}
