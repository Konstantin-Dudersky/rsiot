use serde::Serialize;

#[derive(Clone, Default, Serialize)]
pub struct FunctionBlockBase<I, Q, S>
where
    I: Clone + Default + Serialize,
    Q: Clone + Default + Serialize,
    S: Clone + Default + Serialize,
    Self: IFunctionBlock<I, Q, S>,
{
    pub input: I,
    pub output: Q,
    pub stat: S,
}

impl<I, Q, S> FunctionBlockBase<I, Q, S>
where
    I: Clone + Default + Serialize,
    Q: Clone + Default + Serialize,
    S: Clone + Default + Serialize,
    Self: IFunctionBlock<I, Q, S>,
{
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn call(&mut self, input: I) -> Q {
        self.input = input;
        self.output = FunctionBlockBase::logic(&self.input, &mut self.stat);
        self.output.clone()
    }
}

pub trait IFunctionBlock<I, Q, S> {
    /// Основная логика выполнения блока
    ///
    /// Нужно переопределить для своего функционального блока.
    /// Вызывать самому не нужно, вызывается функцией `call`
    fn logic(input: &I, stat: &mut S) -> Q;
}
