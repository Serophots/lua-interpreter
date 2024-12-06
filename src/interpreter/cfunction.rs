use std::{cell::RefCell, rc::Rc};

use super::genv::GlobalEnv;
use crate::lprimative::LValue;

/// A function written in C, typically made available to
/// interpretted bytecode via the global env
///
/// 'i lifetime lives as long as the interpreter
pub type CFunction<'i> =
    fn(&mut GlobalEnv<'i>, &[Rc<RefCell<LValue<'i>>>]) -> Vec<Rc<RefCell<LValue<'i>>>>;
