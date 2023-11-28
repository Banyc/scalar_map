#![cfg_attr(not(feature = "std"), no_std)]

pub trait ScalarMapExt: Sized {
    fn map<T>(self, f: impl Fn(Self) -> T) -> T {
        f(self)
    }

    fn and_then<T>(self, f: impl Fn(Self) -> T) -> T {
        self.map(f)
    }
}
impl ScalarMapExt for bool {}
impl ScalarMapExt for u8 {}
impl ScalarMapExt for u16 {}
impl ScalarMapExt for u32 {}
impl ScalarMapExt for u64 {}
impl ScalarMapExt for u128 {}
impl ScalarMapExt for usize {}
impl ScalarMapExt for i8 {}
impl ScalarMapExt for i16 {}
impl ScalarMapExt for i32 {}
impl ScalarMapExt for i64 {}
impl ScalarMapExt for i128 {}
impl ScalarMapExt for isize {}
impl ScalarMapExt for f32 {}
impl ScalarMapExt for f64 {}
impl ScalarMapExt for char {}
impl ScalarMapExt for &str {}
#[cfg(feature = "std")]
impl ScalarMapExt for String {}
#[cfg(feature = "std")]
impl ScalarMapExt for &std::path::Path {}
#[cfg(feature = "std")]
impl ScalarMapExt for std::path::PathBuf {}
#[cfg(feature = "std")]
impl<T> ScalarMapExt for std::sync::Arc<T> {}
#[cfg(feature = "std")]
impl<T> ScalarMapExt for std::rc::Rc<T> {}
#[cfg(feature = "std")]
impl<T> ScalarMapExt for std::sync::Mutex<T> {}
#[cfg(feature = "std")]
impl<T> ScalarMapExt for std::cell::RefCell<T> {}
#[cfg(feature = "std")]
impl<K, V> ScalarMapExt for std::collections::HashMap<K, V> {}
#[cfg(feature = "std")]
impl<T> ScalarMapExt for std::collections::HashSet<T> {}
#[cfg(feature = "std")]
impl<K, V> ScalarMapExt for std::collections::BTreeMap<K, V> {}
#[cfg(feature = "std")]
impl<T> ScalarMapExt for std::collections::BTreeSet<T> {}
#[cfg(feature = "std")]
impl<T> ScalarMapExt for std::collections::VecDeque<T> {}
#[cfg(feature = "std")]
impl<T> ScalarMapExt for std::collections::BinaryHeap<T> {}
#[cfg(feature = "std")]
impl<T> ScalarMapExt for std::collections::LinkedList<T> {}
#[cfg(feature = "std")]
impl<T> ScalarMapExt for Vec<T> {}
impl<T> ScalarMapExt for &[T] {}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "std")]
    #[test]
    fn test_map_std() {
        use std::sync::Arc;

        let num: i32 = 42;
        assert_eq!(num.map(Arc::new), Arc::new(42));
    }

    #[test]
    fn test_map() {
        let num: Option<i32> = Some(42);
        assert_eq!(num.map(|x| 42 - x), Some(0));

        let num: i32 = 42;
        assert_eq!(num.map(|x| 42 - x), 0);

        let num: f32 = 1.0;
        assert_eq!(num.map(|x| 1.0 - x), 0.0);

        let hello = "hello";
        assert_eq!(hello.map(|x| x), "hello");
    }

    #[test]
    #[allow(clippy::bind_instead_of_map)]
    fn test_and_then() {
        let num: Option<i32> = Some(42);
        assert_eq!(num.and_then(Option::Some), Some(42));

        let num: i32 = 42;
        assert_eq!(num.and_then(Option::Some), Some(42));
    }

    #[test]
    fn test_custom_struct() {
        #[derive(Debug, PartialEq)]
        struct MyNum(i32);
        impl ScalarMapExt for MyNum {}

        let num = MyNum(42);
        assert_eq!(num.map(|x| 42 - x.0).map(MyNum), MyNum(0));
    }
}
