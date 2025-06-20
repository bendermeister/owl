use std::hash::Hash;
use std::{marker::PhantomData, ops::Deref};

pub trait IDAble {}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub struct ID<T: IDAble> {
    id: u64,
    marker: PhantomData<T>,
}

impl<T: IDAble> Clone for ID<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: IDAble> Copy for ID<T> {}

impl<T: IDAble> Hash for ID<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

impl<T: IDAble> Deref for ID<T> {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.id
    }
}

impl<T: IDAble> ID<T> {
    pub fn new(id: u64) -> Self {
        Self {
            id,
            marker: PhantomData,
        }
    }
}
