pub trait DeriveItemProcess<TMsg>: Send + Sync {
    fn process(&mut self, msg: &TMsg) -> Option<Vec<TMsg>>;
}
