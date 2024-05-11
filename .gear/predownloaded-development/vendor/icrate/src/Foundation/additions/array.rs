//! Utilities for the `NSArray` and `NSMutableArray` classes.
#![cfg(feature = "Foundation_NSArray")]
use alloc::vec::Vec;
use core::fmt;
use core::mem;
use core::ops::{Index, IndexMut, Range};
use core::panic::{RefUnwindSafe, UnwindSafe};

use objc2::mutability::{IsMutable, IsRetainable};
use objc2::rc::IdFromIterator;

use super::iter;
use super::util;
use crate::common::*;
#[cfg(feature = "Foundation_NSMutableArray")]
use crate::Foundation::NSMutableArray;
use crate::Foundation::{self, NSArray};

impl<T: Message> NSArray<T> {
    pub fn from_vec(mut vec: Vec<Id<T>>) -> Id<Self> {
        // We intentionally extract the length before we access the
        // pointer as mutable, to not invalidate that mutable pointer.
        let len = vec.len();
        let ptr = util::id_ptr_cast(vec.as_mut_ptr());
        // SAFETY: We've consumed the `Id<T>`s, which means that we can
        // now safely take ownership (even if `T` is mutable).
        unsafe { Self::initWithObjects_count(Self::alloc(), ptr, len) }
        // The drop of `Vec` here would invalidate our mutable pointer,
        // except for the fact that we're using `UnsafeCell` in `AnyObject`.
    }

    pub fn from_id_slice(slice: &[Id<T>]) -> Id<Self>
    where
        T: IsIdCloneable,
    {
        let len = slice.len();
        let ptr = util::id_ptr_cast_const(slice.as_ptr());
        // SAFETY: Because of the `T: IsIdCloneable` bound, and since we
        // take `&[Id<T>]` (effectively `&Id<T>`), we are allowed to give
        // the slice to Objective-C, which will retain it internally.
        //
        // Faster version of:
        //     Self::from_vec(slice.iter().map(|obj| obj.clone()).collect())
        unsafe { Self::initWithObjects_count(Self::alloc(), ptr, len) }
    }

    pub fn from_slice(slice: &[&T]) -> Id<Self>
    where
        T: IsRetainable,
    {
        let len = slice.len();
        let ptr = util::ref_ptr_cast_const(slice.as_ptr());
        // SAFETY: Because of the `T: IsRetainable` bound, we are allowed
        // to give the slice to Objective-C, which will retain it
        // internally.
        //
        // Faster version of:
        //     Self::from_vec(slice.iter().map(|obj| obj.retain()).collect())
        unsafe { Self::initWithObjects_count(Self::alloc(), ptr, len) }
    }

    #[doc(alias = "getObjects:range:")]
    pub fn to_vec(&self) -> Vec<&T> {
        // SAFETY: The range is know to be in bounds
        unsafe { self.objects_in_range_unchecked(0..self.len()) }
    }

    #[doc(alias = "getObjects:range:")]
    pub fn to_vec_retained(&self) -> Vec<Id<T>>
    where
        T: IsIdCloneable,
    {
        // SAFETY: The objects are stored in the array
        self.to_vec()
            .into_iter()
            .map(|obj| unsafe { util::collection_retain_id(obj) })
            .collect()
    }

    // `fn into_vec(Id<NSArray>) -> Vec<Id<T>>` would not be safe, since
    // the array itself is unconditionally `IsIdCloneable`, even when
    // containing mutable elements, and hence we would be able to
    // duplicate those.
}

#[cfg(feature = "Foundation_NSMutableArray")]
impl<T: Message> NSMutableArray<T> {
    pub fn from_vec(mut vec: Vec<Id<T>>) -> Id<Self> {
        let len = vec.len();
        let ptr = util::id_ptr_cast(vec.as_mut_ptr());
        // SAFETY: Same as `NSArray::from_vec`.
        unsafe { Self::initWithObjects_count(Self::alloc(), ptr, len) }
    }

    pub fn from_id_slice(slice: &[Id<T>]) -> Id<Self>
    where
        T: IsIdCloneable,
    {
        let len = slice.len();
        let ptr = util::id_ptr_cast_const(slice.as_ptr());
        // SAFETY: Same as `NSArray::from_id_slice`
        unsafe { Self::initWithObjects_count(Self::alloc(), ptr, len) }
    }

    pub fn from_slice(slice: &[&T]) -> Id<Self>
    where
        T: IsRetainable,
    {
        let len = slice.len();
        let ptr = util::ref_ptr_cast_const(slice.as_ptr());
        // SAFETY: Same as `NSArray::from_slice`.
        unsafe { Self::initWithObjects_count(Self::alloc(), ptr, len) }
    }

    pub fn into_vec(array: Id<Self>) -> Vec<Id<T>> {
        // SAFETY: We've consumed the array, so taking ownership of the
        // returned values is safe.
        array
            .to_vec()
            .into_iter()
            .map(|obj| unsafe { util::mutable_collection_retain_removed_id(obj) })
            .collect()
    }
}

impl<T: Message> NSArray<T> {
    #[doc(alias = "count")]
    pub fn len(&self) -> usize {
        self.count()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

extern_methods!(
    unsafe impl<T: Message> NSArray<T> {
        #[method(objectAtIndex:)]
        unsafe fn get_unchecked(&self, index: usize) -> &T;

        #[doc(alias = "objectAtIndex:")]
        pub fn get(&self, index: usize) -> Option<&T> {
            // TODO: Replace this check with catching the thrown NSRangeException
            if index < self.len() {
                // SAFETY: The index is checked to be in bounds.
                Some(unsafe { self.get_unchecked(index) })
            } else {
                None
            }
        }

        #[doc(alias = "objectAtIndex:")]
        pub fn get_retained(&self, index: usize) -> Option<Id<T>>
        where
            T: IsIdCloneable,
        {
            // SAFETY: The object is stored in the array
            self.get(index)
                .map(|obj| unsafe { util::collection_retain_id(obj) })
        }

        #[method(objectAtIndex:)]
        unsafe fn get_unchecked_mut(&mut self, index: usize) -> &mut T;

        #[doc(alias = "objectAtIndex:")]
        pub fn get_mut(&mut self, index: usize) -> Option<&mut T>
        where
            T: IsMutable,
        {
            // TODO: Replace this check with catching the thrown NSRangeException
            if index < self.len() {
                // SAFETY: The index is checked to be in bounds, and the
                // reference is safe as mutable because of the `T: IsMutable`
                // bound.
                Some(unsafe { self.get_unchecked_mut(index) })
            } else {
                None
            }
        }

        #[doc(alias = "firstObject")]
        #[method(firstObject)]
        pub fn first(&self) -> Option<&T>;

        #[doc(alias = "firstObject")]
        pub fn first_retained(&self) -> Option<Id<T>>
        where
            T: IsIdCloneable,
        {
            // SAFETY: The object is stored in the array
            self.first()
                .map(|obj| unsafe { util::collection_retain_id(obj) })
        }

        #[doc(alias = "firstObject")]
        #[method(firstObject)]
        pub fn first_mut(&mut self) -> Option<&mut T>
        where
            T: IsMutable;

        #[doc(alias = "lastObject")]
        #[method(lastObject)]
        pub fn last(&self) -> Option<&T>;

        #[doc(alias = "lastObject")]
        pub fn last_retained(&self) -> Option<Id<T>>
        where
            T: IsIdCloneable,
        {
            // SAFETY: The object is stored in the array
            self.last()
                .map(|obj| unsafe { util::collection_retain_id(obj) })
        }

        #[doc(alias = "lastObject")]
        #[method(lastObject)]
        pub fn last_mut(&mut self) -> Option<&mut T>
        where
            T: IsMutable;
    }
);

impl<T: Message> NSArray<T> {
    unsafe fn objects_in_range_unchecked(&self, range: Range<usize>) -> Vec<&T> {
        let range = Foundation::NSRange::from(range);
        let mut vec: Vec<NonNull<T>> = Vec::with_capacity(range.length);
        unsafe {
            self.getObjects_range(NonNull::new(vec.as_mut_ptr()).unwrap(), range);
            vec.set_len(range.length);
            mem::transmute(vec)
        }
    }

    #[doc(alias = "getObjects:range:")]
    pub fn objects_in_range(&self, range: Range<usize>) -> Option<Vec<&T>> {
        if range.end > self.len() {
            return None;
        }
        // SAFETY: Just checked that the range is in bounds
        Some(unsafe { self.objects_in_range_unchecked(range) })
    }
}

#[cfg(feature = "Foundation_NSMutableArray")]
impl<T: Message> NSMutableArray<T> {
    #[doc(alias = "addObject:")]
    pub fn push(&mut self, obj: Id<T>) {
        // SAFETY: We've consumed ownership of the object.
        unsafe { self.addObject(&obj) }
    }

    #[doc(alias = "insertObject:atIndex:")]
    pub fn insert(&mut self, index: usize, obj: Id<T>) {
        // TODO: Replace this check with catching the thrown NSRangeException
        let len = self.len();
        if index < len {
            // SAFETY: We've consumed ownership of the object, and the
            // index is checked to be in bounds.
            unsafe { self.insertObject_atIndex(&obj, index) }
        } else {
            panic!(
                "insertion index (is {}) should be <= len (is {})",
                index, len
            );
        }
    }

    #[doc(alias = "replaceObjectAtIndex:withObject:")]
    pub fn replace(&mut self, index: usize, obj: Id<T>) -> Result<Id<T>, Id<T>> {
        if let Some(old_obj) = self.get(index) {
            // SAFETY: We remove the object from the array below.
            let old_obj = unsafe { util::mutable_collection_retain_removed_id(old_obj) };
            // SAFETY: The index is checked to be in bounds, and we've
            // consumed ownership of the new object.
            unsafe { self.replaceObjectAtIndex_withObject(index, &obj) };
            Ok(old_obj)
        } else {
            Err(obj)
        }
    }

    #[doc(alias = "removeObjectAtIndex:")]
    pub fn remove(&mut self, index: usize) -> Option<Id<T>> {
        let obj = self.get(index)?;
        // SAFETY: We remove the object from the array below.
        let obj = unsafe { util::mutable_collection_retain_removed_id(obj) };
        // SAFETY: The index is checked to be in bounds.
        unsafe { self.removeObjectAtIndex(index) };
        Some(obj)
    }

    #[doc(alias = "removeLastObject")]
    pub fn pop(&mut self) -> Option<Id<T>> {
        let obj = self.last()?;
        // SAFETY: We remove the object from the array below.
        let obj = unsafe { util::mutable_collection_retain_removed_id(obj) };
        // SAFETY: Just checked that there is an object.
        unsafe { self.removeLastObject() };
        Some(obj)
    }

    #[doc(alias = "sortUsingFunction:context:")]
    pub fn sort_by<F: FnMut(&T, &T) -> core::cmp::Ordering>(&mut self, compare: F) {
        // TODO: "C-unwind"
        unsafe extern "C" fn compare_with_closure<T, F: FnMut(&T, &T) -> core::cmp::Ordering>(
            obj1: NonNull<T>,
            obj2: NonNull<T>,
            context: *mut c_void,
        ) -> isize {
            let context: *mut F = context.cast();
            // Bring back a reference to the closure.
            // Guaranteed to be unique, we gave `sortUsingFunction` unique is
            // ownership, and that method only runs one function at a time.
            let closure: &mut F = unsafe { context.as_mut().unwrap_unchecked() };

            // SAFETY: The objects are guaranteed to be valid
            let (obj1, obj2) = unsafe { (obj1.as_ref(), obj2.as_ref()) };

            Foundation::NSComparisonResult::from((*closure)(obj1, obj2)) as _
        }

        // Create function pointer
        let f: unsafe extern "C" fn(_, _, _) -> _ = compare_with_closure::<T, F>;

        // Grab a type-erased pointer to the closure (a pointer to stack).
        let mut closure = compare;
        let context: *mut F = &mut closure;

        unsafe { self.sortUsingFunction_context(f, context.cast()) };
        // Keep the closure alive until the function has run.
        drop(closure);
    }
}

impl<T: Message> NSArray<T> {
    #[doc(alias = "objectEnumerator")]
    #[inline]
    pub fn iter(&self) -> Iter<'_, T> {
        Iter(super::iter::Iter::new(self))
    }

    #[doc(alias = "objectEnumerator")]
    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<'_, T>
    where
        T: IsMutable,
    {
        IterMut(super::iter::IterMut::new(self))
    }

    #[doc(alias = "objectEnumerator")]
    #[inline]
    pub fn iter_retained(&self) -> IterRetained<'_, T>
    where
        T: IsIdCloneable,
    {
        IterRetained(super::iter::IterRetained::new(self))
    }
}

unsafe impl<T: Message> iter::FastEnumerationHelper for NSArray<T> {
    type Item = T;

    #[inline]
    fn maybe_len(&self) -> Option<usize> {
        Some(self.len())
    }
}

#[cfg(feature = "Foundation_NSMutableArray")]
unsafe impl<T: Message> iter::FastEnumerationHelper for NSMutableArray<T> {
    type Item = T;

    #[inline]
    fn maybe_len(&self) -> Option<usize> {
        Some(self.len())
    }
}

/// An iterator over the items of a `NSArray`.
#[derive(Debug)]
pub struct Iter<'a, T: Message>(iter::Iter<'a, NSArray<T>>);

__impl_iter! {
    impl<'a, T: Message> Iterator<Item = &'a T> for Iter<'a, T> { ... }
}

/// A mutable iterator over the items of a `NSArray`.
#[derive(Debug)]
pub struct IterMut<'a, T: Message>(iter::IterMut<'a, NSArray<T>>);

__impl_iter! {
    impl<'a, T: IsMutable> Iterator<Item = &'a mut T> for IterMut<'a, T> { ... }
}

/// An iterator that retains the items of a `NSArray`.
#[derive(Debug)]
pub struct IterRetained<'a, T: Message>(iter::IterRetained<'a, NSArray<T>>);

__impl_iter! {
    impl<'a, T: IsIdCloneable> Iterator<Item = Id<T>> for IterRetained<'a, T> { ... }
}

/// A consuming iterator over the items of a `NSArray`.
#[derive(Debug)]
pub struct IntoIter<T: Message>(iter::IntoIter<NSArray<T>>);

__impl_iter! {
    impl<'a, T: Message> Iterator<Item = Id<T>> for IntoIter<T> { ... }
}

__impl_into_iter! {
    impl<T: Message> IntoIterator for &NSArray<T> {
        type IntoIter = Iter<'_, T>;
    }

    #[cfg(feature = "Foundation_NSMutableArray")]
    impl<T: Message> IntoIterator for &NSMutableArray<T> {
        type IntoIter = Iter<'_, T>;
    }

    impl<T: IsMutable> IntoIterator for &mut NSArray<T> {
        type IntoIter = IterMut<'_, T>;
    }

    #[cfg(feature = "Foundation_NSMutableArray")]
    impl<T: IsMutable> IntoIterator for &mut NSMutableArray<T> {
        type IntoIter = IterMut<'_, T>;
    }

    impl<T: IsIdCloneable> IntoIterator for Id<NSArray<T>> {
        type IntoIter = IntoIter<T>;
    }

    #[cfg(feature = "Foundation_NSMutableArray")]
    impl<T: Message> IntoIterator for Id<NSMutableArray<T>> {
        type IntoIter = IntoIter<T>;
    }
}

impl<T: Message> Index<usize> for NSArray<T> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        self.get(index).unwrap()
    }
}

#[cfg(feature = "Foundation_NSMutableArray")]
impl<T: Message> Index<usize> for NSMutableArray<T> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        self.get(index).unwrap()
    }
}

impl<T: IsMutable> IndexMut<usize> for NSArray<T> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        self.get_mut(index).unwrap()
    }
}

#[cfg(feature = "Foundation_NSMutableArray")]
impl<T: IsMutable> IndexMut<usize> for NSMutableArray<T> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        self.get_mut(index).unwrap()
    }
}

impl<T: fmt::Debug + Message> fmt::Debug for NSArray<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self).finish()
    }
}

#[cfg(feature = "Foundation_NSMutableArray")]
impl<T: Message> Extend<Id<T>> for NSMutableArray<T> {
    fn extend<I: IntoIterator<Item = Id<T>>>(&mut self, iter: I) {
        iter.into_iter().for_each(move |item| self.push(item))
    }
}

#[cfg(feature = "Foundation_NSMutableArray")]
impl<'a, T: IsRetainable> Extend<&'a T> for NSMutableArray<T> {
    fn extend<I: IntoIterator<Item = &'a T>>(&mut self, iter: I) {
        // SAFETY: Because of the `T: IsRetainable` bound, it is safe for the
        // array to retain the object here.
        iter.into_iter()
            .for_each(move |item| unsafe { self.addObject(item) })
    }
}

impl<'a, T: IsRetainable + 'a> IdFromIterator<&'a T> for NSArray<T> {
    fn id_from_iter<I: IntoIterator<Item = &'a T>>(iter: I) -> Id<Self> {
        let vec = Vec::from_iter(iter);
        Self::from_slice(&vec)
    }
}

impl<T: Message> IdFromIterator<Id<T>> for NSArray<T> {
    fn id_from_iter<I: IntoIterator<Item = Id<T>>>(iter: I) -> Id<Self> {
        let vec = Vec::from_iter(iter);
        Self::from_vec(vec)
    }
}

#[cfg(feature = "Foundation_NSMutableArray")]
impl<'a, T: IsRetainable + 'a> IdFromIterator<&'a T> for NSMutableArray<T> {
    fn id_from_iter<I: IntoIterator<Item = &'a T>>(iter: I) -> Id<Self> {
        let vec = Vec::from_iter(iter);
        Self::from_slice(&vec)
    }
}

#[cfg(feature = "Foundation_NSMutableArray")]
impl<T: Message> IdFromIterator<Id<T>> for NSMutableArray<T> {
    fn id_from_iter<I: IntoIterator<Item = Id<T>>>(iter: I) -> Id<Self> {
        let vec = Vec::from_iter(iter);
        Self::from_vec(vec)
    }
}
