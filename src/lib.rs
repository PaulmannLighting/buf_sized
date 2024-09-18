mod impls;

#[cfg(feature = "derive")]
pub use buf_sized_derive::BufSized;

pub trait BufSized {
    const BUF_SIZE: usize;
}

#[must_use]
pub const fn buf_size<T: BufSized>() -> usize {
    T::BUF_SIZE
}
