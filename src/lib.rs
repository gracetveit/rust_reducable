//! # Reducable
//!
//! Provides a trait that allows you to apply the `reduce` function to a struct.
//!
//! ## Example
//!
//! ```rust
//! use reduce::Reducable;
//!
//! struct A<T> {
//!     value1: T,
//!     value2: T,
//! }
//!
//! impl<T> Reducable<T> for A<T> {
//!     fn reduce_function<R>(&self, fnc: fn(acc: R, cur: &T) -> R, initial: R) -> R
//!     {
//!         fnc(fnc(initial, &self.value1), &self.value2)
//!     }
//! }
//!
//! let a = A{
//!     value1: 1,
//!     value2: 2
//! };
//!
//! let sum = a.reduce(|acc, cur| -> i32 {acc + cur}, None);
//! assert_eq!(sum, 3)
//!```

/// Reducable
pub trait Reducable<T> {
    /// The reduce function to be called
    ///
    /// # Examples
    /// ```rust
    /// use reduce::Reducable;
    ///
    /// let vec_sum = vec![1, 2, 3, 4, 5];
    /// let sum = vec_sum.reduce(|acc, cur| -> i32 {acc + cur}, None);
    /// assert_eq!(sum, 15)
    /// ```
    fn reduce<R>(&self, fnc: fn(acc: R, cur: &T) -> R, initial: Option<R>) -> R
    where
        R: Default,
    {
        let unpacked_initial: R = get_initial(initial);
        self.reduce_function(fnc, unpacked_initial)
    }

    /// The function to define the reduce logic for a given structure
    fn reduce_function<R>(&self, fnc: fn(acc: R, cur: &T) -> R, initial_value: R) -> R;
}

fn get_initial<R>(initial: Option<R>) -> R
where
    R: Default,
{
    match initial {
        Some(x) => x,
        None => Default::default(),
    }
}

impl<T> Reducable<T> for Vec<T> {
    fn reduce_function<R>(&self, fnc: fn(acc: R, cur: &T) -> R, initial_value: R) -> R {
        let mut return_value = initial_value;
        for x in self {
            return_value = fnc(return_value, x);
        }
        return_value
    }
}
