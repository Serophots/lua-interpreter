use std::{cell::RefCell, rc::Rc};

use closure::LClosure;

use self::genv::GlobalEnv;
use crate::{
    bytecode::bproto::BProto,
    lprimative::{LPrimitive, LValue},
};

pub mod cfunction;
pub mod closure;
pub mod genv;

pub type StackItem<'i> = Rc<RefCell<LValue<'i>>>;
pub type Stack<'i> = Vec<StackItem<'i>>; //Rc<RefCell<>> temporary garbage collector I guess

pub struct Interpreter<'i> {
    genv: GlobalEnv<'i>,
    stack: Stack<'i>,
    top: Box<BProto>,
}
impl<'i> Interpreter<'i> {
    pub fn new(top: Box<BProto>) -> Interpreter<'i> {
        let mut stack = Vec::with_capacity(top.max_stack as usize);

        for _ in 0..top.max_stack {
            stack.push(Rc::new(RefCell::new(LValue::LPrimitive(LPrimitive::NIL))));
        }

        Self {
            genv: GlobalEnv::default(),
            stack,
            top,
        }
    }

    pub fn interpret(&'i mut self) {
        //Instantiate a closure for the top proto
        let mut top_closure = LClosure::<'i>::new(&self.top, vec![]);

        //Call the top closure
        top_closure.execute(&mut self.genv, &mut self.stack, 0);
    }
}
