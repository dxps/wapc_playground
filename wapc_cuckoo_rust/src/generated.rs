extern crate rmp_serde as rmps;
use rmps::{Deserializer, Serializer};
use serde::{Deserialize, Serialize};
use std::io::Cursor;

#[cfg(feature = "guest")]
extern crate wapc_guest as guest;
#[cfg(feature = "guest")]
use guest::prelude::*;

#[cfg(feature = "guest")]
pub struct Host {
    binding: String,
}

#[cfg(feature = "guest")]
impl Default for Host {
    fn default() -> Self {
        Host {
            binding: "default".to_string(),
        }
    }
}

/// Creates a named host binding
#[cfg(feature = "guest")]
pub fn host(binding: &str) -> Host {
    Host {
        binding: binding.to_string(),
    }
}

/// Creates the default host binding
#[cfg(feature = "guest")]
pub fn default() -> Host {
    Host::default()
}

#[cfg(feature = "guest")]
impl Host {
    pub fn check_word_exists(&self, name: String) -> HandlerResult<String> {
        let input_args = Check_word_existsArgs { name };
        host_call(
            &self.binding,
            "cuckoo",
            "check_word_exists",
            &serialize(input_args)?,
        )
        .map(|vec| {
            let resp = deserialize::<String>(vec.as_ref()).unwrap();
            resp
        })
        .map_err(|e| e.into())
    }
    pub fn handle_input(&self, inp: Input) -> HandlerResult<String> {
        let input_args = Handle_inputArgs { inp };
        host_call(
            &self.binding,
            "cuckoo",
            "handle_input",
            &serialize(input_args)?,
        )
        .map(|vec| {
            let resp = deserialize::<String>(vec.as_ref()).unwrap();
            resp
        })
        .map_err(|e| e.into())
    }
}

#[cfg(feature = "guest")]
pub struct Handlers {}

#[cfg(feature = "guest")]
impl Handlers {
    pub fn register_check_word_exists(f: fn(String) -> HandlerResult<String>) {
        *CHECK_WORD_EXISTS.write().unwrap() = Some(f);
        register_function(&"check_word_exists", check_word_exists_wrapper);
    }
    pub fn register_handle_input(f: fn(Input) -> HandlerResult<String>) {
        *HANDLE_INPUT.write().unwrap() = Some(f);
        register_function(&"handle_input", handle_input_wrapper);
    }
}

#[cfg(feature = "guest")]
lazy_static::lazy_static! {
static ref CHECK_WORD_EXISTS: std::sync::RwLock<Option<fn(String) -> HandlerResult<String>>> = std::sync::RwLock::new(None);
static ref HANDLE_INPUT: std::sync::RwLock<Option<fn(Input) -> HandlerResult<String>>> = std::sync::RwLock::new(None);
}

#[cfg(feature = "guest")]
fn check_word_exists_wrapper(input_payload: &[u8]) -> CallResult {
    let input = deserialize::<Check_word_existsArgs>(input_payload)?;
    let lock = CHECK_WORD_EXISTS.read().unwrap().unwrap();
    let result = lock(input.name)?;
    serialize(result)
}

#[cfg(feature = "guest")]
fn handle_input_wrapper(input_payload: &[u8]) -> CallResult {
    let input = deserialize::<Handle_inputArgs>(input_payload)?;
    let lock = HANDLE_INPUT.read().unwrap().unwrap();
    let result = lock(input.inp)?;
    serialize(result)
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct Check_word_existsArgs {
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct Handle_inputArgs {
    #[serde(rename = "inp")]
    pub inp: Input,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct Input {
    #[serde(rename = "x")]
    pub x: String,
    #[serde(rename = "y")]
    pub y: String,
}

/// The standard function for serializing codec structs into a format that can be
/// used for message exchange between actor and host. Use of any other function to
/// serialize could result in breaking incompatibilities.
pub fn serialize<T>(
    item: T,
) -> ::std::result::Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>>
where
    T: Serialize,
{
    let mut buf = Vec::new();
    item.serialize(&mut Serializer::new(&mut buf).with_struct_map())?;
    Ok(buf)
}

/// The standard function for de-serializing codec structs from a format suitable
/// for message exchange between actor and host. Use of any other function to
/// deserialize could result in breaking incompatibilities.
pub fn deserialize<'de, T: Deserialize<'de>>(
    buf: &[u8],
) -> ::std::result::Result<T, Box<dyn std::error::Error + Send + Sync>> {
    let mut de = Deserializer::new(Cursor::new(buf));
    match Deserialize::deserialize(&mut de) {
        Ok(t) => Ok(t),
        Err(e) => Err(format!("Failed to de-serialize: {}", e).into()),
    }
}
