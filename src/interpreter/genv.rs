use crate::lprimative::LValue;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::cfunction::CFunction;

/// 'i lifetime lives as long as the interpretter
pub struct GlobalEnv<'i> {
    pub(crate) items: HashMap<String, CFunction<'i>>,
}
impl<'i> Default for GlobalEnv<'i> {
    fn default() -> GlobalEnv<'i> {
        let mut items: HashMap<String, CFunction<'i>> = HashMap::new();

        items.insert("print".to_owned(), c_print);

        Self { items }
    }
}

pub fn c_print<'i>(
    _genv: &mut GlobalEnv<'i>,
    args: &[Rc<RefCell<LValue<'i>>>],
) -> Vec<Rc<RefCell<LValue<'i>>>> {
    print!(" >> PRINT >> ");
    for a in args {
        print!("{} ", a.borrow());
    }
    print!("\n");

    return Vec::new();
}
