/// A slice with at least N elements. Can be dereferenced back into a regular slice on demand.
///
/// ```
/// use cl_array_ext::SliceN;
/// let a: &mut [_] = &mut [1, 2, 3, 4, 5];
/// let b: &mut SliceN<_, 3> = a.try_into().unwrap();
///
/// b.head = [3, 2, 1];
/// b.tail.reverse();
///
/// assert_eq!(a, [3, 2, 1, 5, 4]);
/// ```
#[repr(C)]
#[allow(clippy::module_name_repetitions)]
pub struct SliceN<T, const N: usize> {
    /// Head of the slice, where N items are guaranteed to exist
    pub head: [T; N],
    /// Tail of the slice, may be empty
    pub tail: [T],
}

impl<T, const N: usize> SliceN<T, N> {
    /// Convert a slice into one that is guaranteed to have at least N elements
    /// # Panics
    /// The length of the slice must be >= N, otherwise this will panic
    pub fn from_unchecked(slice: &[T]) -> &Self {
        // extract the pointer metadata for the slice
        let (p, meta) = (slice as *const [T]).to_raw_parts();
        // convert the address and meta back into a ref
        unsafe { &*core::ptr::from_raw_parts(p, meta - N) }
    }

    /// Convert a mut slice into one that is guaranteed to have at least N elements
    /// # Panics
    /// The length of the slice must be >= N, otherwise this will panic
    pub fn from_unchecked_mut(slice: &mut [T]) -> &mut Self {
        // extract the pointer metadata for the slice
        let (p, meta) = (slice as *mut [T]).to_raw_parts();
        // convert the address and meta back into a ref
        unsafe { &mut *core::ptr::from_raw_parts_mut(p, meta - N) }
    }
}

/// Error type returned by the [`TryFrom`] implementations for [`SliceN`]
#[derive(Debug)]
pub struct NotEnoughEntries;

impl<'a, T, const N: usize> TryFrom<&'a [T]> for &'a SliceN<T, N> {
    type Error = NotEnoughEntries;
    fn try_from(value: &'a [T]) -> Result<Self, Self::Error> {
        if value.len() < N {
            Err(NotEnoughEntries)
        } else {
            Ok(SliceN::<T, N>::from_unchecked(value))
        }
    }
}

impl<'a, T, const N: usize> TryFrom<&'a mut [T]> for &'a mut SliceN<T, N> {
    type Error = NotEnoughEntries;
    fn try_from(value: &'a mut [T]) -> Result<Self, Self::Error> {
        if value.len() < N {
            Err(NotEnoughEntries)
        } else {
            Ok(SliceN::<T, N>::from_unchecked_mut(value))
        }
    }
}

use core::fmt;
impl<T: fmt::Debug, const N: usize> fmt::Debug for SliceN<T, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list()
            .entries(self.iter())
            .finish()
    }
}

use core::ops::{Deref, DerefMut};
impl<T, const N: usize> Deref for SliceN<T, N> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        // extract the pointer metadata for the slice
        let (p, meta) = (self as *const SliceN<T, N>).to_raw_parts();
        // convert the address and meta back into a ref
        unsafe { &*core::ptr::from_raw_parts(p, meta + N) }
    }
}

impl<T, const N: usize> DerefMut for SliceN<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // extract the pointer metadata for the slice
        let (p, meta) = (self as *mut SliceN<T, N>).to_raw_parts();
        // convert the address and meta back into a ref
        unsafe { &mut *core::ptr::from_raw_parts_mut(p, meta + N) }
    }
}

#[cfg(test)]
mod tests {
    use crate::SliceN;

    #[test]
    fn slice_n() {
        let a: &[_] = &[1, 2, 3, 4, 5];
        let b: &SliceN<_, 3> = a.try_into().unwrap();

        assert_eq!(b.len(), 5);
        assert_eq!(b.head, [1, 2, 3]);
        assert_eq!(b.tail, [4, 5]);
        assert_eq!(&**b, a);

        let _ = <&SliceN<_, 6>>::try_from(a).unwrap_err();
    }

    #[test]
    fn slice_n_mut() {
        let a: &mut [_] = &mut [1, 2, 3, 4, 5];
        let b: &mut SliceN<_, 3> = a.try_into().unwrap();

        b.head = [3, 2, 1];

        assert_eq!(a, [3, 2, 1, 4, 5]);
    }
}
