//! holochain_dna::zome::capabilities is a set of structs for working with holochain dna.

use std::str::FromStr;
use wasm::DnaWasm;

//--------------------------------------------------------------------------------------------------
// Reserved Capabilities and functions names
//--------------------------------------------------------------------------------------------------

#[derive(Debug, PartialEq)]
/// Enumeration of all Capabilities known and used by HC Core
/// Enumeration converts to str
pub enum ReservedCapabilityNames {
    /// Development placeholder, no production fn should use MissingNo
    MissingNo,

    /// @TODO document what LifeCycle is
    /// @see https://github.com/holochain/holochain-rust/issues/204
    LifeCycle,

    /// @TODO document what Communication is
    /// @see https://github.com/holochain/holochain-rust/issues/204
    Communication,
}

impl FromStr for ReservedCapabilityNames {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "hc_lifecycle" => Ok(ReservedCapabilityNames::LifeCycle),
            "hc_web_gateway" => Ok(ReservedCapabilityNames::Communication),
            _ => Err("Cannot convert string to ReservedCapabilityNames"),
        }
    }
}

impl ReservedCapabilityNames {
    pub fn as_str(&self) -> &'static str {
        match *self {
            ReservedCapabilityNames::LifeCycle => "hc_lifecycle",
            ReservedCapabilityNames::Communication => "hc_web_gateway",
            ReservedCapabilityNames::MissingNo => "",
        }
    }
}

//--------------------------------------------------------------------------------------------------
//
//--------------------------------------------------------------------------------------------------

/// Enum for Zome Capability "membrane" property.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Hash)]
pub enum Membrane {
    #[serde(rename = "public")]
    Public,
    #[serde(rename = "agent")]
    Agent,
    #[serde(rename = "api-key")]
    ApiKey,
    #[serde(rename = "zome")]
    Zome,
}

impl Default for Membrane {
    /// Default zome capability membrane is "agent"
    fn default() -> Self {
        Membrane::Agent
    }
}

/// Represents the "capability" sub-object on a "zome" "capabilities" object.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Hash)]
pub struct CapabilityType {
    /// How visibility should be handled for this capability.
    #[serde(default)]
    pub membrane: Membrane,
}

impl Default for CapabilityType {
    /// Defaults for a "capability" sub-object on a "zome" "capabilities" object.
    fn default() -> Self {
        CapabilityType {
            membrane: Membrane::Agent,
        }
    }
}

impl CapabilityType {
    /// Allow sane defaults for `CapabilityType::new()`.
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Hash)]
pub struct FnParameter {
    #[serde(rename = "type")]
    pub parameter_type: String,
    pub name: String,
}

impl FnParameter {
    #[allow(dead_code)]
    fn new<S: Into<String>>(n: S, t: S) -> FnParameter {
        FnParameter {
            name: n.into(),
            parameter_type: t.into(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Hash)]
pub struct FnSignature {
    pub inputs: Vec<FnParameter>,
    pub outputs: Vec<FnParameter>,
}

/// Represents a zome "fn_declarations" object.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Hash)]
pub struct FnDeclaration {
    /// The name of this fn declaration.
    #[serde(default)]
    pub name: String,
    pub signature: FnSignature,
}

impl Default for FnDeclaration {
    /// Defaults for a "fn_declarations" object.
    fn default() -> Self {
        FnDeclaration {
            name: String::from(""),
            signature: FnSignature {
                inputs: Vec::new(),
                outputs: Vec::new(),
            },
        }
    }
}

impl FnDeclaration {
    /// Allow sane defaults for `FnDecrlaration::new()`.
    pub fn new() -> Self {
        Default::default()
    }
}

/// Represents an individual object in the "zome" "capabilities" array.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Hash)]
pub struct Capability {
    /// The name of this capability.
    #[serde(default)]
    pub name: String,

    /// "capability" sub-object
    #[serde(default)]
    pub capability: CapabilityType,

    /// "fn_declarations" array
    #[serde(default)]
    pub fn_declarations: Vec<FnDeclaration>,

    /// Validation code for this entry_type.
    #[serde(default)]
    pub code: DnaWasm,
}

impl Default for Capability {
    /// Provide defaults for a "zome"s "capabilities" object.
    fn default() -> Self {
        Capability {
            name: String::from(""),
            capability: CapabilityType::new(),
            fn_declarations: Vec::new(),
            code: DnaWasm::new(),
        }
    }
}

impl Capability {
    /// Allow sane defaults for `Capability::new()`.
    pub fn new() -> Self {
        Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    /// test that ReservedCapabilityNames can be created from a canonical string
    fn test_capabilities_from_str() {
        assert_eq!(
            Ok(ReservedCapabilityNames::LifeCycle),
            ReservedCapabilityNames::from_str("hc_lifecycle"),
        );
        assert_eq!(
            Ok(ReservedCapabilityNames::Communication),
            ReservedCapabilityNames::from_str("hc_web_gateway"),
        );
        assert_eq!(
            Err("Cannot convert string to ReservedCapabilityNames"),
            ReservedCapabilityNames::from_str("foo"),
        );
    }

    #[test]
    /// test that a canonical string can be created from ReservedCapabilityNames
    fn test_capabilities_as_str() {
        assert_eq!(ReservedCapabilityNames::LifeCycle.as_str(), "hc_lifecycle");
        assert_eq!(
            ReservedCapabilityNames::Communication.as_str(),
            "hc_web_gateway",
        );
    }

    #[test]
    fn build_and_compare() {
        let fixture: Capability = serde_json::from_str(
            r#"{
                "name": "test",
                "capability": {
                    "membrane": "agent"
                },
                "fn_declarations": [
                    {
                        "name": "test",
                        "signature":
                        {
                            "inputs": [
                                {
                                    "name": "post",
                                    "type": "string"
                                }
                            ],
                            "outputs": [
                                {
                                    "name": "hash",
                                    "type": "string"
                                }
                            ]
                        }
                    }
                ],
                "code": {
                    "code": "AAECAw=="
                }
            }"#,
        ).unwrap();

        let mut cap = Capability::new();
        cap.name = String::from("test");
        let mut fn_dec = FnDeclaration::new();
        fn_dec.name = String::from("test");
        let input = FnParameter::new("post", "string");
        let output = FnParameter::new("hash", "string");
        fn_dec.signature.inputs.push(input);
        fn_dec.signature.outputs.push(output);
        cap.fn_declarations.push(fn_dec);
        cap.code.code = vec![0, 1, 2, 3];

        assert_eq!(fixture, cap);
    }
}
