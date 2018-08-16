use nucleus::ribosome::api::Runtime;
use wasmi::{RuntimeArgs, RuntimeValue, Trap};

pub fn invoke_call(
    runtime: &mut Runtime,
    args: &RuntimeArgs,
) -> Result<Option<RuntimeValue>, Trap> {
    Ok(None)
}
