use crate::BufSized;

impl<const SIZE: usize, T> BufSized for [T; SIZE]
where
    T: BufSized,
{
    const BUF_SIZE: usize = SIZE * T::BUF_SIZE;
}

#[cfg(feature = "heapless")]
impl<const SIZE: usize, T> BufSized for heapless::Vec<T, SIZE>
where
    T: BufSized,
{
    const BUF_SIZE: usize = SIZE * T::BUF_SIZE;
}

#[cfg(test)]
mod tests {
    use crate::BufSized;

    #[test]
    fn test_array_u8() {
        assert_eq!(<[u8; 10]>::BUF_SIZE, 10);
    }

    #[test]
    fn test_array_u16() {
        assert_eq!(<[u16; 10]>::BUF_SIZE, 20);
    }
    #[test]
    fn test_array_u32() {
        assert_eq!(<[u32; 10]>::BUF_SIZE, 40);
    }

    #[cfg(feature = "heapless")]
    mod heapless {
        use crate::BufSized;

        #[test]
        fn test_vec_u8() {
            assert_eq!(heapless::Vec::<u8, 10>::BUF_SIZE, 10);
        }

        #[test]
        fn test_vec_u16() {
            assert_eq!(heapless::Vec::<u16, 10>::BUF_SIZE, 20);
        }

        #[test]
        fn test_vec_u32() {
            assert_eq!(heapless::Vec::<u32, 10>::BUF_SIZE, 40);
        }
    }
}
