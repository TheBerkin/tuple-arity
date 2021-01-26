/// Trait for getting the arity of a value. You shouldn't need to impl this yourself.
pub trait Arity {
    /// Gets the arity of the type.
    ///
    /// Example
    /// ```rust
    /// use tuple_arity::Arity;
    /// 
    /// assert_eq!(0, <()>::ARITY);
    /// assert_eq!(1, <(u8,)>::ARITY);
    /// assert_eq!(2, <(u8, u8)>::ARITY);
    /// assert_eq!(3, <(u8, u8, u8)>::ARITY);
    /// assert_eq!(4, <(u8, u8, u8, u8)>::ARITY);
    /// ```
    const ARITY: usize;


    /// Gets the arity of the type.
    ///
    /// Example
    /// ```rust
    /// use tuple_arity::Arity;
    /// 
    /// assert_eq!(0, <()>::arity());
    /// assert_eq!(1, <(u8,)>::arity());
    /// assert_eq!(2, <(u8, u8)>::arity());
    /// assert_eq!(3, <(u8, u8, u8)>::arity());
    /// assert_eq!(4, <(u8, u8, u8, u8)>::arity());
    /// ```
    #[deprecated(
        since = "0.1.3",
        note = "Use the associated constant `ARITY` instead",
    )]
    #[inline(always)]
    fn arity() -> usize {
        Self::ARITY
    }
}

macro_rules! impl_tuple_arity {
    ($len:expr, $($tuple_arg:ident),*) => {
        impl<$($tuple_arg,)*> Arity for ($($tuple_arg,)*) {
            const ARITY: usize = $len;
        }
    }
}

/// Gets the arity of the specified value.
///
/// # Example
/// ```rust
/// use tuple_arity::*;
/// assert_eq!(0, tuple_arity(&()));
/// assert_eq!(1, tuple_arity(&("foo",)));
/// assert_eq!(2, tuple_arity(&("foo", "bar")));
/// assert_eq!(3, tuple_arity(&("foo", "bar", "baz")));
/// ```
#[inline(always)]
pub fn tuple_arity<T: Arity>(_: &T) -> usize {
    T::ARITY
}


impl_tuple_arity!(0,);
impl_tuple_arity!(1, T1);
impl_tuple_arity!(2, T1, T2);
impl_tuple_arity!(3, T1, T2, T3);
impl_tuple_arity!(4, T1, T2, T3, T4);
impl_tuple_arity!(5, T1, T2, T3, T4, T5);
impl_tuple_arity!(6, T1, T2, T3, T4, T5, T6);
impl_tuple_arity!(7, T1, T2, T3, T4, T5, T6, T7);
impl_tuple_arity!(8, T1, T2, T3, T4, T5, T6, T7, T8);
impl_tuple_arity!(9, T1, T2, T3, T4, T5, T6, T7, T8, T9);
impl_tuple_arity!(10, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
impl_tuple_arity!(11, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
impl_tuple_arity!(12, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);