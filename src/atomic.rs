use core::marker::PhantomData;
use core::sync::atomic::{AtomicPtr, Ordering};

use crate::Ring;

pub struct AtomicCursor<T> {
    ptr: AtomicPtr<T>,
    marker: PhantomData<T>,
}

impl<T> AtomicCursor<T> {
    /// Constructs new atomic cursor given a `Cursor`.
    #[inline]
    pub fn new(cursor: Cursor<T>) -> Self {
        let ptr = AtomicPtr::new(cursor.ptr().as_ptr());
        Self {
            ptr,
            marker: PhantomData,
        }
    }

    #[inline]
    pub fn load(&self, order: Ordering) -> Cursor<T> {
        let ptr = self.ptr.load(order);
        unsafe { Cursor::new_unchecked(ptr) }
    }

    #[inline]
    pub fn store(&self, cursor: Cursor<T>, order: Ordering) {
        self.ptr.store(cursor.ptr().as_ptr(), order);
    }

    #[inline]
    pub fn advance(&self, ring: &Ring<T>, order: Ordering) {
        let cursor = self.load(order);
        let cursor = ring.advance(cursor);
        self.store(cursor, order);
    }
}

impl<T> fmt::Debug for AtomicCursor<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AtomicCursor({:?})", self.ptr)
    }
}

impl<T> From<Cursor<T>> for AtomicCursor<T> {
    fn from(cursor: Cursor<T>) -> Self {
        cursor.into_atomic()
    }
}
