extern crate stable_deref_trait;
use self::stable_deref_trait::StableDeref;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Default, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct NoCallback;

pub trait CallbackSafe {
    type Target: ?Sized;
    fn as_stable_mut(&mut self) -> &mut Self::Target;
}

static mut UNIT: () = ();

impl Deref for NoCallback {
    type Target = ();
    fn deref(&self) -> &Self::Target {
        unsafe { &UNIT }
    }
}

impl DerefMut for NoCallback {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut UNIT }
    }
}

unsafe impl StableDeref for NoCallback {}

impl<T, Q: StableDeref<Target = T> + DerefMut<Target = T>> CallbackSafe for Q {
    type Target = T;
    fn as_stable_mut(&mut self) -> &mut Self::Target {
        self.deref_mut()
    }
}
