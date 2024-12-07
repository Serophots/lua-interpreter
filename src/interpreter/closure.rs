use std::{cell::RefCell, rc::Rc};

use crate::{
    bytecode::{binstruction::BInstruction, bproto::BProto},
    lprimative::{LPrimitive, LValue},
};

use super::{genv::GlobalEnv, Stack, StackItem};

macro_rules! Kst {
    ($proto:expr, $n:expr) => {
        $proto
            .constants
            .list
            .get($n as usize)
            .expect("No constant exists at Kst!() lookup")
    };
}
macro_rules! genv {
    ($genv:expr, $n:expr) => {
        $genv.items.get($n).expect("Couldn't find genv!() lookup")
    };
}

/// For each call of a proto, a new Closure is instantiated
/// and stored on the stack to capture some upvalue context
/// of the particular call.
///
/// 'i lifetime lives as long as the Interpreter does
#[derive(Debug)]
pub struct LClosure<'i> {
    proto: &'i Box<BProto>,
    upvalues: Vec<Rc<RefCell<LValue<'i>>>>,
}
impl<'i> LClosure<'i> {
    /// A closure is instantiated by CLOSURE instruction followed by
    /// either MOVE or GETUPVAL instructions corresponding to each upvalue
    /// the proto needs. In this manor upvalues are stored in the closure
    /// rather than the closure having to maintain references to its parents
    ///
    /// The upvalue LValue's must be reference variants to LValue's owned by
    /// stacks of parent closures
    pub fn new(proto: &'i Box<BProto>, upvalues: Vec<Rc<RefCell<LValue<'i>>>>) -> LClosure<'i> {
        assert_eq!(
            proto.num_upvalues as usize,
            upvalues.len(),
            "Closure instantiated with unexpected number of upvalues"
        );

        Self { proto, upvalues }
    }

    pub fn execute(
        &mut self,
        genv: &'i mut GlobalEnv<'i>,
        stack: &'i mut Stack<'i>,
        base: usize,
    ) -> &'i [StackItem<'i>] {
        //Instruction execution
        let mut pc = 0;

        loop {
            let instruction = self
                .proto
                .instructions
                .list
                .get(pc)
                .expect(format!("no instruction found at pc={}", pc).as_str());

            println!("executing {:?}", instruction);

            match *instruction {
                BInstruction::ABC { opcode, a, b, c } => {
                    let a = a as usize;
                    let b = b as usize;
                    let c = c as usize;
                    match opcode {
                        0 => {
                            // MOVE
                            //  Copies the value of register R(B) into register R(A). If R(B) holds a table,
                            //  function or userdata, then the reference to that object is copied. MOVE is
                            //  often used for moving values into place for the next operation.
                            //
                            //  The opcode for MOVE has a second purpose – it is also used in creating
                            //  closures, always appearing after the CLOSURE instruction; see CLOSURE
                            //  for more information.

                            let b = match &*stack[base + b].borrow() {
                                //By value
                                LValue::LPrimitive(p) => {
                                    Rc::new(RefCell::new(LValue::LPrimitive(p.clone())))
                                }
                                //By reference
                                _ => todo!("MOVE by reference or unhandled"),
                            };

                            stack[base + a] = b;
                        }
                        28 => {
                            // CALL
                            //  Performs a function call, with register R(A) holding the reference to the
                            //  function object to be called. Parameters to the function are placed in the
                            //  registers following R(A). If B is 1, the function has no parameters. If B is 2
                            //  or more, there are (B-1) parameters.
                            //
                            //  If B is 0, the function parameters range from R(A+1) to the top of the stack.
                            //  This form is used when the last expression in the parameter list is a
                            //  function call, so the number of actual parameters is indeterminate.
                            //
                            //  Results returned by the function call is placed in a range of registers
                            //  starting from R(A). If C is 1, no return results are saved. If C is 2 or more,
                            //  (C-1) return values are saved. If C is 0, then multiple return results are
                            //  saved, depending on the called function.
                            //
                            //  CALL always updates the top of stack value. CALL, RETURN, VARARG
                            //  and SETLIST can use multiple values (up to the top of the stack.)

                            match b {
                                0 => {
                                    //Params from a+1 to top
                                    todo!("call with indeterminate parameters");
                                }
                                b => {
                                    //b-1 params

                                    match &*stack[base + a].borrow() {
                                        LValue::CClosure(c) => {
                                            c(genv, &stack[base + a + 1..base + a + b])
                                        }
                                        LValue::LClosure(_) => todo!("call with LClosure"),
                                        _ => panic!("CALL called with non-closure"),
                                    };
                                }
                            }
                        }
                        30 => {
                            // RETURN
                            //  Returns to the calling function, with optional return values. If B is 1, there
                            //  are no return values. If B is 2 or more, there are (B-1) return values,
                            //  located in consecutive registers from R(A) onwards.
                            //
                            //  If B is 0, the set of values from R(A) to the top of the stack is returned. This
                            //  form is used when the last expression in the return list is a function call, so
                            //  the number of actual values returned is indeterminate.
                            //
                            //  RETURN also closes any open upvalues, equivalent to a CLOSE
                            //  instruction. See the CLOSE instruction for more information.

                            return match b {
                                0 => {
                                    //Params from a to top
                                    todo!("return with indeterminate parameters");
                                }
                                1 => {
                                    //No return values
                                    &[]
                                }
                                b => {
                                    //From a to b-1
                                    &stack[base + a..base + b - 1]
                                }
                            };
                        }
                        _ => todo!("instruction unhandled: {:?}", instruction),
                    }
                }
                BInstruction::ABx { opcode, a, b } => {
                    let a = a as usize;
                    let b = b as usize;
                    match opcode {
                        1 => {
                            // LOADK
                            //  Loads constant number Bx into register R(A). Constants are usually
                            //  numbers or strings. Each function has its own constant list, or pool.

                            stack[base + a] = Rc::new(RefCell::new(LValue::LPrimitive(
                                Kst!(self.proto, b).clone(),
                            )));
                        }
                        5 => {
                            // GETGLOBAL
                            //  Copies the value of the global variable whose name is given in constant
                            //  number Bx into register R(A). The name constant must be a string.

                            if let LPrimitive::STRING(c) = Kst!(self.proto, b) {
                                stack[base + a] =
                                    Rc::new(RefCell::new(LValue::CClosure(genv!(genv, c).clone())));
                            } else {
                                panic!("GETGLOBAL Kst!(b) points to non-STRING primitive");
                            }
                        }
                        _ => todo!("instruction unhandled: {:?}", instruction),
                    }
                }
                BInstruction::AsBx { opcode, a, b } => match opcode {
                    0 => {
                        todo!()
                    }
                    _ => todo!("instruction unhandled: {:?}", instruction),
                },
            }

            pc += 1;
        }
    }
}

//                             //CALL

//                             //We want a &mut to a on the stack as well as a &[] of elements after a - borrow checker says no
//                             let (stack_a, stack_post_a) = self.stack.split_at_mut(a as usize + 1);

//                             let params = match b {
//                                 0 => {
//                                     //Params from a+1 to stack_top
//                                     &stack_post_a[..(self.stack_top + 1 - stack_a.len())]
//                                     //Inclusive..exclusive
//                                     //                                    &self.stack[(a as usize + 1) .. (self.stack_top + 1)] //inclusive exclusive
//                                 }
//                                 1 => {
//                                     //No params
//                                     &[]
//                                 }
//                                 b => {
//                                     //Params from a+1 to b-1
//                                     &stack_post_a[..(b as usize - stack_a.len())]
//                                     //                                    &self.stack[(a as usize +1)..(b as usize)] //Inclusive exclusive
//                                 }
//                             };

//                             let _results = match R_mut!(stack_a, a) {
//                                 LValue::LClosure(closure) => closure.call(genv, params),
//                                 LValue::CClosure(function) => function(genv, params),
//                                 _ => panic!("CALL on non-closure register at R!(a)"),
//                             };
//                         }
//                         30 => {
//                             //RETURN
//                             //todo CLOSE upvalues bit

//                             return match b {
//                                 0 => {
//                                     //From a to stack_top
//                                     self.stack.drain(a..(self.stack_top + 1)).collect()
//                                 }
//                                 1 => {
//                                     //No return values
//                                     Vec::new()
//                                 }
//                                 b => {
//                                     //From a to b-1
//                                     self.stack.drain(a..b).collect()
//                                 }
//                             };
//                         }
//                         _ => panic!(
//                             "Instruction of ABC type has unhandled opcode: {} a: {} b: {} c: {}",
//                             opcode, a, b, c
//                         ),
//                     }
//                 }
