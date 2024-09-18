//! A library for types with a fixed buffer size.
mod impls;

#[cfg(feature = "derive")]
pub use buf_sized_derive::BufSized;

/// A trait for types that have a fixed buffer size.
pub trait BufSized {
    /// The buffer size of the type.
    const BUF_SIZE: usize;
}

/// Get the buffer size of a type.
#[must_use]
pub const fn buf_size<T: BufSized>() -> usize {
    T::BUF_SIZE
}
