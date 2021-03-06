use action::{Action, ActionWrapper};
use agent::state::ActionResponse;
use nucleus::ribosome::api::{
    runtime_allocate_encode_str, runtime_args_to_utf8, HcApiReturnCode, Runtime,
};
use serde_json;
use std::sync::mpsc::channel;
use wasmi::{RuntimeArgs, RuntimeValue, Trap};

#[derive(Deserialize, Default, Debug, Serialize)]
struct GetArgs {
    key: String,
}

pub fn invoke_get(runtime: &mut Runtime, args: &RuntimeArgs) -> Result<Option<RuntimeValue>, Trap> {
    // deserialize args
    let args_str = runtime_args_to_utf8(&runtime, &args);
    let res_entry: Result<GetArgs, _> = serde_json::from_str(&args_str);
    // Exit on error
    if res_entry.is_err() {
        // Return Error code in i32 format
        return Ok(Some(RuntimeValue::I32(
            HcApiReturnCode::ErrorSerdeJson as i32,
        )));
    }

    let input = res_entry.unwrap();

    let action_wrapper = ActionWrapper::new(Action::Get(input.key));

    let (sender, receiver) = channel();
    ::instance::dispatch_action_with_observer(
        &runtime.action_channel,
        &runtime.observer_channel,
        action_wrapper.clone(),
        move |state: &::state::State| {
            let mut actions_copy = state.agent().actions();
            match actions_copy.remove(&action_wrapper) {
                Some(v) => {
                    // @TODO never panic in wasm
                    // @see https://github.com/holochain/holochain-rust/issues/159
                    sender
                        .send(v)
                        // the channel stays connected until the first message has been sent
                        // if this fails that means that it was called after having returned done=true
                        .expect("observer called after done");

                    true
                }
                None => false,
            }
        },
    );
    // TODO #97 - Return error if timeout or something failed
    // return Err(_);

    let action_result = receiver.recv().expect("observer dropped before done");

    match action_result {
        ActionResponse::Get(maybe_pair) => {
            // serialize, allocate and encode result
            let pair_str = maybe_pair.map(|p| p.to_json()).unwrap_or_default();

            runtime_allocate_encode_str(runtime, &pair_str)
        }
        _ => Ok(Some(RuntimeValue::I32(
            HcApiReturnCode::ErrorActionResult as i32,
        ))),
    }
}

#[cfg(test)]
mod tests {
    extern crate test_utils;
    extern crate wabt;

    use super::GetArgs;
    use hash_table::entry::tests::{test_entry, test_entry_hash};
    use nucleus::ribosome::api::tests::test_zome_api_function_runtime;
    use serde_json;

    /// dummy get args from standard test entry
    pub fn test_args_bytes() -> Vec<u8> {
        let args = GetArgs {
            key: test_entry().hash().into(),
        };
        serde_json::to_string(&args).unwrap().into_bytes()
    }

    #[test]
    /// test that we can round trip bytes through a get action and it comes back from wasm
    fn test_get_round_trip() {
        let (runtime, _) = test_zome_api_function_runtime("get", test_args_bytes());

        let mut expected = "".to_owned();
        expected.push_str("{\"header\":{\"entry_type\":\"testEntryType\",\"time\":\"\",\"next\":null,\"entry\":\"");
        expected.push_str(&test_entry_hash());
        expected.push_str("\",\"type_next\":null,\"signature\":\"\"},\"entry\":{\"content\":\"test entry content\",\"entry_type\":\"testEntryType\"}}\u{0}");

        assert_eq!(runtime.result, expected);
    }

}
