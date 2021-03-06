//! Some array extensions that really have no purpose other than to make me feel accomplisheds

#![no_std]
#![allow(incomplete_features)]
#![feature(generic_const_exprs, ptr_metadata)]
#![warn(clippy::pedantic, missing_docs)]

mod slice;
pub use slice::*;

/// Trait that extends upon [array]
pub trait ArrayExt<T, const N: usize>: Sized {
    /// Split an array into two smaller arrays
    ///
    /// ```
    /// use cl_array_ext::ArrayExt;
    /// let (a, b) = [1_i32, 2, 3, 4, 5].array_split_at::<3>();
    /// assert_eq!(a, [1, 2, 3]);
    /// assert_eq!(b, [4, 5]);
    /// ```
    fn array_split_at<const M: usize>(self) -> ([T; M], [T; N - M])
    where
        [T; N - M]: Sized;

    /// Take only M elements out of the array
    ///
    /// ```
    /// use cl_array_ext::ArrayExt;
    /// let a = [1, 2, 3, 4, 5].truncate::<3>();
    /// assert_eq!(a, [1, 2, 3]);
    /// ```
    fn truncate<const M: usize>(self) -> [T; M]
    where
        [T; N - M]: Sized,
    {
        self.array_split_at().0
    }

    /// Join two arrays into one larger array
    ///
    /// ```
    /// use cl_array_ext::ArrayExt;
    /// let a = [1_i32, 2, 3].append([4, 5]);
    /// assert_eq!(a, [1, 2, 3, 4, 5]);
    /// ```
    fn append<const M: usize>(self, other: [T; M]) -> [T; N + M];
}

impl<T, const N: usize> ArrayExt<T, N> for [T; N] {
    fn array_split_at<const M: usize>(self) -> ([T; M], [T; N - M])
    where
        [T; N - M]: Sized,
    {
        let arr = core::mem::ManuallyDrop::new(self);
        unsafe {
            (
                core::ptr::read(arr.as_ptr().add(0).cast()),
                core::ptr::read(arr.as_ptr().add(M).cast()),
            )
        }
    }

    fn append<const M: usize>(self, other: [T; M]) -> [T; N + M] {
        let arr_a = core::mem::ManuallyDrop::new(self);
        let arr_b = core::mem::ManuallyDrop::new(other);
        let mut arr_c = core::mem::MaybeUninit::<[T; N + M]>::uninit();
        let p = arr_c.as_mut_ptr().cast::<T>();

        unsafe {
            core::ptr::copy(arr_a.as_ptr(), p.add(0), N);
            core::ptr::copy(arr_b.as_ptr(), p.add(N), M);

            core::mem::MaybeUninit::assume_init(arr_c)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ArrayExt;

    #[test]
    fn split_at() {
        let (a, b) = [1, 2, 3, 4, 5].array_split_at::<3>();
        assert_eq!(a, [1, 2, 3]);
        assert_eq!(b, [4, 5]);
    }

    #[test]
    fn append() {
        let a = [1, 2, 3].append([4, 5]);

        assert_eq!(a, [1, 2, 3, 4, 5]);
    }
}
