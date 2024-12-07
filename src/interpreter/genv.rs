use crate::lprimative::LValue;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::cfunction::{CFunction, CProto};

/// 'i lifetime lives as long as the interpretter
pub struct GlobalEnv<'i> {
    pub(crate) items: HashMap<String, CFunction<'i>>,
}
impl<'i> Default for GlobalEnv<'i> {
    fn default() -> GlobalEnv<'i> {
        let mut items: HashMap<String, CFunction<'i>> = HashMap::new();

        items.insert(
            "print".to_owned(),
            (
                CProto {
                    num_params: 1,
                    vararg_flag: 0,
                },
                c_print,
            ),
        );

        Self { items }
    }
}

/// Prints to the top of stack, so all varargs, fixed args, etc
pub fn c_print<'i>(
    _genv: &mut GlobalEnv<'i>,
    args: &[Rc<RefCell<LValue<'i>>>],
) -> Vec<Rc<RefCell<LValue<'i>>>> {
    print!(" >> PRINT >> ");
    for a in &args[1..] {
        print!("{} ", a.borrow());
    }
    print!("\n");

    return Vec::new();
}
