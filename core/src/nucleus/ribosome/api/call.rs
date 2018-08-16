use nucleus::ribosome::api::Runtime;
use wasmi::{RuntimeArgs, RuntimeValue, Trap};

pub fn invoke_call(
    runtime: &mut Runtime,
    _args: &RuntimeArgs,
) -> Result<Option<RuntimeValue>, Trap> {
    let _context = runtime.context.clone();

    Ok(None)
}
