use std::marker::PhantomData;

#[derive(Clone)]
pub struct Config<TMessage> {
    _p: PhantomData<TMessage>,
}
