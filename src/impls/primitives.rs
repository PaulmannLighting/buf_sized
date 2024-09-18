use crate::BufSized;

impl BufSized for u8 {
    const BUF_SIZE: usize = (Self::BITS / 8) as usize;
}

impl BufSized for u16 {
    const BUF_SIZE: usize = (Self::BITS / 8) as usize;
}

impl BufSized for u32 {
    const BUF_SIZE: usize = (Self::BITS / 8) as usize;
}

impl BufSized for u64 {
    const BUF_SIZE: usize = (Self::BITS / 8) as usize;
}

impl BufSized for u128 {
    const BUF_SIZE: usize = (Self::BITS / 8) as usize;
}

impl BufSized for i8 {
    const BUF_SIZE: usize = (Self::BITS / 8) as usize;
}

impl BufSized for i16 {
    const BUF_SIZE: usize = (Self::BITS / 8) as usize;
}

impl BufSized for i32 {
    const BUF_SIZE: usize = (Self::BITS / 8) as usize;
}

impl BufSized for i64 {
    const BUF_SIZE: usize = (Self::BITS / 8) as usize;
}

impl BufSized for f32 {
    const BUF_SIZE: usize = 4;
}

impl BufSized for f64 {
    const BUF_SIZE: usize = 8;
}

#[cfg(test)]
mod tests {
    use crate::BufSized;

    #[test]
    fn test_u8() {
        assert_eq!(u8::BUF_SIZE, 1);
    }

    #[test]
    fn test_u16() {
        assert_eq!(u16::BUF_SIZE, 2);
    }

    #[test]
    fn test_u32() {
        assert_eq!(u32::BUF_SIZE, 4);
    }

    #[test]
    fn test_u64() {
        assert_eq!(u64::BUF_SIZE, 8);
    }

    #[test]
    fn test_u128() {
        assert_eq!(u128::BUF_SIZE, 16);
    }

    #[test]
    fn test_i8() {
        assert_eq!(i8::BUF_SIZE, 1);
    }

    #[test]
    fn test_i16() {
        assert_eq!(i16::BUF_SIZE, 2);
    }

    #[test]
    fn test_i32() {
        assert_eq!(i32::BUF_SIZE, 4);
    }

    #[test]
    fn test_i64() {
        assert_eq!(i64::BUF_SIZE, 8);
    }

    #[test]
    fn test_f32() {
        assert_eq!(f32::BUF_SIZE, 4);
    }

    #[test]
    fn test_f64() {
        assert_eq!(f64::BUF_SIZE, 8);
    }
}
