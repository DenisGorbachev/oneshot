use derive_more::{Display, Error, From};
use serde::Serialize;

use crate::format::Format;

pub fn serialize<T: Serialize>(
    #[allow(unused_variables)] value: &T,
    format: Format,
) -> Result<String, SerializeError> {
    let output = match format {
        #[cfg(feature = "serde_json")]
        Format::Json => serde_json::to_string_pretty(value)?,
        #[cfg(feature = "serde_yaml")]
        Format::Yaml => serde_yaml::to_string(value)?,
        #[cfg(feature = "serde_xml_rs")]
        Format::Xml => serde_xml_rs::to_string(value)?,
        #[allow(unreachable_patterns)]
        _ => String::new(),
    };
    Ok(output)
}

#[derive(Error, Display, From, Debug)]
#[non_exhaustive]
pub enum SerializeError {
    #[cfg(feature = "serde_json")]
    Json(serde_json::Error),
    #[cfg(feature = "serde_yaml")]
    Yaml(serde_yaml::Error),
    #[cfg(feature = "serde_xml_rs")]
    Xml(serde_xml_rs::Error),
}
