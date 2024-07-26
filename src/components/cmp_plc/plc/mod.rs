//! Модули, связанные с работой plc

mod function_block_base;
pub mod library;
pub mod types;

pub use function_block_base::{FbSystemData, FunctionBlockBase, IFunctionBlock};
