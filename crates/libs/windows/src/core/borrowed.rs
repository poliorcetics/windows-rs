use super::*;

// TODO: this also needs to merge with IntoParam/Param

#[repr(transparent)]
pub struct Borrowed<'a, T: Abi + 'a>(T::Abi, std::marker::PhantomData<&'a T>);

impl<T: Abi> Default for Borrowed<'static, T> {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl<'a, T: Abi + 'a> Borrowed<'a, T> {
    pub fn new(target: &T) -> Self {
        Self(unsafe { std::mem::transmute_copy(target) }, std::marker::PhantomData)
    }
}

impl<'a, T: Abi + 'a> std::ops::Deref for Borrowed<'a, T> {
    type Target = T::DefaultType;
    fn deref(&self) -> &T::DefaultType {
        unsafe { std::mem::transmute(&self.0) }
    }
}
