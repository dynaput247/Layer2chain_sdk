use crate::{error::ZomeApiResult, Dispatch};
use holochain_json_api::json::JsonString;

/// Prints a string through the stdout of the running Conductor, and also
/// writes that string to the logger in the execution context
/// # Examples
/// ```rust
/// # #[macro_use]
/// # extern crate hdk;
/// # use hdk::error::ZomeApiResult;
///
/// pub fn handle_some_function(content: String) -> ZomeApiResult<()> {
///     // ...
///     hdk::debug("write a message to the logs");
///     // ...
///     Ok(())
/// }
/// ```
pub fn debug<J: Into<String>>(msg: J) -> ZomeApiResult<()> {
    let _: ZomeApiResult<()> = Dispatch::Debug.with_input(JsonString::from_json(&msg.into()));
    // internally returns RibosomeEncodedValue::Success which is a zero length allocation
    // return Ok(()) unconditionally instead of the "error" from success
    Ok(())
}
