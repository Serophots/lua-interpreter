use std::{cell::RefCell, rc::Rc};

use crate::{
    bytecode::{binstruction::BInstruction, bproto::BProto},
    lprimative::{LPrimitive, LValue},
};

use super::{genv::GlobalEnv, Stack};

macro_rules! Kst {
    ($proto:expr, $n:expr) => {
        $proto
            .constants
            .list
            .get($n as usize)
            .expect("No constant exists at Kst!() lookup")
    };
}
macro_rules! Proto {
    ($proto:expr, $n:expr) => {
        $proto
            .protos
            .list
            .get($n as usize)
            .expect("No proto exists at Proto!() lookup")
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
    proto: &'i BProto,
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
    pub fn new(proto: &'i BProto, upvalues: Vec<Rc<RefCell<LValue<'i>>>>) -> LClosure<'i> {
        Self { proto, upvalues }
    }

    pub fn execute(
        &self,
        genv: &mut GlobalEnv<'i>,
        // Begins at and includes the Closure being called. Following
        // that come varargs then fixed args. The base is the offset
        // from the bottom of the stack to where the first fixed arg begins
        // The stack must not be a slice because this function needs to be able to extend the underlying Vector as it sees fit
        stack: &mut Stack<'i>,

        top: usize,  //Top index of the stack for this function
        base: usize, //Base index of the stack for this function. The index of the first fixed argument
        func: usize, //Index of the current LClosure/CClosure being executed on the stack. Between this and base are variable arguments
    ) {
        //Instruction execution
        let mut pc = 0;

        loop {
            let instruction = self
                .proto
                .instructions
                .list
                .get(pc)
                .expect(format!("no instruction found at pc={}", pc).as_str());

            println!(
                "executing func={} base={} top={} {:?}",
                func, base, top, instruction
            );

            match *instruction {
                BInstruction::ABC {
                    line,
                    opcode,
                    a,
                    b,
                    c,
                } => {
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
                                LValue::LClosure(_) => stack[base + b].clone(),
                                _ => todo!("MOVE by reference or unhandled"),
                            };

                            stack[base + a] = b;
                        }
                        2 => {
                            // LOADBOOL
                            //  Loads a boolean value (true or false) into register R(A). true is usually
                            //  encoded as an integer 1, false is always 0. If C is non-zero, then the next
                            //  instruction is skipped (this is used when you have an assignment
                            //  statement where the expression uses relational operators, e.g. M = K>5.)
                            //
                            //  You can use any non-zero value for the boolean true in field B, but since
                            //  you cannot use booleans as numbers in Lua, it’s best to stick to 1 for true.
                            // let b = match &*stack[base + b].borrow() {
                            //     //By value
                            //     LValue::LPrimitive(p) => {
                            //         Rc::new(RefCell::new(LValue::LPrimitive(p.clone())))
                            //     }
                            //     //By reference
                            //     _ => todo!("MOVE by reference or unhandled"),
                            // };

                            stack[base + a] =
                                Rc::new(RefCell::new(LValue::LPrimitive(LPrimitive::BOOL(b != 0))));

                            if c != 0 {
                                pc += 1;
                            }
                        }
                        opcode @ 12..=17 => {
                            // ADD, SUB, MUL, DIV, MOD, POW
                            // let lhs: f64 = if b < 256 {
                            //     match &*stack[base + b].borrow() {
                            //         LValue::LPrimitive(LPrimitive::NUMBER(x)) => *x,
                            //         _ => panic!(
                            //             "ADD failed: Left hand side stack lvalue is not a number"
                            //         ),
                            //     }
                            // } else {
                            //     match Kst!(self.proto, (b - 256)) {
                            //         LPrimitive::NUMBER(x) => *x,
                            //         _ => panic!("ADD failed: Left hand side constant lprimitive is not a number"),
                            //     }
                            // };
                            // let rhs: f64 = if c < 256 {
                            //     match &*stack[base + c].borrow() {
                            //         LValue::LPrimitive(LPrimitive::NUMBER(x)) => *x,
                            //         _ => panic!(
                            //             "ADD failed: Right hand side stack lvalue is not a number"
                            //         ),
                            //     }
                            // } else {
                            //     match Kst!(self.proto, (c - 256)) {
                            //             LPrimitive::NUMBER(x) => *x,
                            //             _ => panic!("ADD failed: Right hand side constant lprimitive is not a number"),
                            //         }
                            // };

                            // stack[base + a] = Rc::new(RefCell::new(LValue::LPrimitive(
                            //     LPrimitive::NUMBER(match opcode {
                            //         12 => lhs + rhs,
                            //         13 => lhs - rhs,
                            //         14 => lhs * rhs,
                            //         15 => lhs / rhs,
                            //         16 => lhs % rhs,
                            //         17 => lhs.powf(rhs),
                            //         _ => {
                            //             unreachable!("opcodes 12..=17 are numerical operations")
                            //         }
                            //     }),
                            // )));
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

                            let register_a = stack[base + a].clone();

                            match &*register_a.borrow() {
                                LValue::CClosure(ref function) => {
                                    let (proto, closure) = function;

                                    let num_args = match b {
                                        0 => top - b - 1,
                                        b => b - 1,
                                    };

                                    println!("num_args, {}", num_args);

                                    let _num_varargs = num_args - proto.num_params as usize;

                                    let call_stack = &stack[base + a..];
                                    closure(genv, call_stack);
                                }
                                LValue::LClosure(ref closure) => {
                                    assert!(
                                        closure.proto.vararg_flag == 0
                                            || closure.proto.vararg_flag == 3,
                                        "legacy vararg syntax unsupported"
                                    );

                                    let num_args = match b {
                                        0 => {
                                            todo!("CALL with variable number of arguments (top)")
                                        }
                                        b => b - 1,
                                    };

                                    let num_varargs = num_args - closure.proto.num_params as usize; //TODO something here. For now just verbose

                                    let closure_func = base + a;
                                    let closure_base = closure_func + 1 + num_varargs;
                                    let closure_top =
                                        closure_base + closure.proto.max_stack as usize;

                                    println!(
                                        "num_args={} num_varargs={} c_func={}, c_base={}, c_top={} max_stack={}",
                                        num_args,
                                        num_varargs,
                                        closure_func,
                                        closure_base,
                                        closure_top,
                                        closure.proto.max_stack,
                                    );

                                    //Ensure the stack is large enough
                                    let delta_stack = closure_top - stack.len();
                                    if delta_stack > 0 {
                                        println!("extending stack by {}", delta_stack);
                                        for _ in 0..delta_stack {
                                            stack.push(Rc::new(RefCell::new(LValue::default())));
                                        }
                                    }

                                    closure.execute(genv, stack, top, closure_base, closure_func);
                                }
                                _ => panic!("CALL called with non-closure"),
                            };
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

                            // TODO: Close any open upvalues? is our makeshift gc gonna handle that for us
                            //

                            match b {
                                0 => {
                                    //Params from a to top
                                    todo!("return with indeterminate parameters");
                                }
                                1 => {
                                    //No return values - clear the stack
                                    for i in func..top {
                                        stack[i] = Rc::new(RefCell::new(LValue::default()));
                                    }
                                }
                                b => {
                                    //Return values are from R(A) to R(B-1). Move them down to the func index
                                    let mut j = func;
                                    for i in base + a..base + a + b - 1 {
                                        stack[j] = Rc::new(RefCell::new(stack[i].take()));
                                        j += 1;
                                    }

                                    //From a to b-1
                                    // for a in &stack[base + a..base + b - 1] {}

                                    // &stack[base + a..base + b - 1]
                                    // todo!("return should write its returns to the stack?");
                                }
                            }

                            return;
                        }
                        _ => todo!("instruction unhandled: {:?}", instruction),
                    }
                }
                BInstruction::ABx { line, opcode, a, b } => {
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
                        3 => {
                            // LOADNIL
                            //  Sets a range of registers from R(A) to R(B) to nil. If a single register is to
                            //  be assigned to, then R(A) = R(B). When two or more consecutive locals
                            //  need to be assigned nil values, only a single LOADNIL is needed.

                            for i in a..=b {
                                stack[base + i] = Rc::new(RefCell::new(LValue::default()));
                            }
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
                        36 => {
                            // CLOSURE
                            //  Creates an instance (or closure) of a function. Bx is the function number of
                            //  the function to be instantiated in the table of function prototypes. This table
                            //  is located after the constant table for each function in a binary chunk. The
                            //  first function prototype is numbered 0. Register R(A) is assigned the
                            //  reference to the instantiated function object.
                            //
                            //  For each upvalue used by the instance of the function KPROTO[Bx], there
                            //  is a pseudo-instruction that follows CLOSURE. Each upvalue corresponds
                            //  to either a MOVE or a GETUPVAL pseudo-instruction. Only the B field on
                            //  either of these pseudo-instructions are significant.
                            //
                            //  A MOVE corresponds to local variable R(B) in the current lexical block,
                            //  which will be used as an upvalue in the instantiated function. A
                            //  GETUPVAL corresponds upvalue number B in the current lexical block.
                            //  The VM uses these pseudo-instructions to manage upvalues.

                            //Fetch the proto to CLOSURE
                            let proto = Proto!(self.proto, b);

                            //Prepare upvalues

                            //Create the closure
                            stack[base + a] = Rc::new(RefCell::new(LValue::LClosure(
                                LClosure::new(proto, vec![]),
                            )));
                        }
                        37 => {
                            // VARARG
                            //  VARARG implements the vararg operator ‘...’ in expressions. VARARG
                            //  copies B-1 parameters into a number of registers starting from R(A),
                            //  padding with nils if there aren’t enough values. If B is 0, VARARG copies
                            //  as many values as it can based on the number of parameters passed. If a
                            //  fixed number of values is required, B is a value greater than 1. If any
                            //  number of values is required, B is 0.

                            match b {
                                0 => {
                                    //Copy varargs starting after func and ending before base
                                    let mut j = base + a;

                                    // println!("stack {:#?}", stack);

                                    for i in func + 1..base {
                                        // println!("moving i={} to j={}", i, j);
                                        stack[j] = stack[i].clone();
                                        j += 1;
                                    }
                                }
                                b => {
                                    //Copy (b-1) varargs starting after func
                                    todo!("VARARG with b-1 unsupported")
                                }
                            }
                        }
                        _ => todo!("instruction unhandled: {:?}", instruction),
                    }
                }
                BInstruction::AsBx { line, opcode, a, b } => match opcode {
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
