use std::ops::Deref;
use std::ops::DerefMut;
use std::sync::Arc;
use std::rc::Rc;

pub trait Inherit<T: ?Sized>: AsRef<T> + AsMut<T> + Deref<Target = T> + DerefMut<Target = T> {}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
