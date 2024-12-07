use super::{genv::GlobalEnv, Stack, StackItem};

/// A function not written in lua made available to Lua
/// via the global environment.
///
/// 'i lifetime lives as long as the interpreter
pub type CClosure<'i> = fn(&mut GlobalEnv<'i>, &[StackItem<'i>]) -> Stack<'i>;

/// Describes features of a CClosure such as its parameters
/// and returns
#[derive(Debug, Clone)]
pub struct CProto {
    pub(crate) num_params: u8,
    pub(crate) vararg_flag: u8,
}

pub type CFunction<'i> = (CProto, CClosure<'i>);
