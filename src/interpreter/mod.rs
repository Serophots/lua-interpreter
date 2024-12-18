use std::{cell::RefCell, rc::Rc};

use lclosure::LClosure;

use self::genv::GlobalEnv;
use crate::{
    bytecode::bproto::BProto,
    lprimative::{LPrimitive, LValue},
};

pub mod cfunction;
pub mod genv;
pub mod lclosure;

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
            stack.push(Rc::new(RefCell::new(LValue::default())));
        }

        Self {
            genv: GlobalEnv::default(),
            stack,
            top,
        }
    }

    pub fn interpret(&'i mut self) {
        //Instantiate a closure for the top proto
        let top_closure = LClosure::<'i>::new(&self.top, vec![]);

        //Call the top closure
        assert_eq!(self.top.max_stack as usize, self.stack.len());

        top_closure.execute(
            &mut self.genv,
            &mut self.stack,
            self.top.max_stack as usize,
            0,
            0,
        );
    }
}
