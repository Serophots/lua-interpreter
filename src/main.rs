use bytecode::decode_bytecode;
use interpreter::Interpreter;

pub(crate) mod bytecode;
pub(crate) mod interpreter;
pub(crate) mod lprimative;

fn main() -> Result<(), anyhow::Error> {
    let mut interpreter = Interpreter::new(decode_bytecode()?);

    interpreter.interpret();

    Ok(())
}
