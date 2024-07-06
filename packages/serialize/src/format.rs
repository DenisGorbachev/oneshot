#[derive(
    Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Copy, Debug, serde::Serialize, serde::Deserialize,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "strum", derive(strum::Display))]
#[non_exhaustive]
pub enum Format {
    #[cfg(feature = "serde_json")]
    Json,
    #[cfg(feature = "serde_yaml")]
    Yaml,
    #[cfg(feature = "serde_xml_rs")]
    Xml,
}

impl Format {
    pub fn to_file_extension(&self) -> &'static str {
        match self {
            #[cfg(feature = "serde_json")]
            Format::Json => "json",
            #[cfg(feature = "serde_yaml")]
            Format::Yaml => "yaml",
            #[cfg(feature = "serde_xml_rs")]
            Format::Xml => "xml",
            #[allow(unreachable_patterns)]
            _ => "txt",
        }
    }

    pub fn get_file_name(&self, stem: &str) -> String {
        format!("{stem}.{extension}", extension = self.to_file_extension())
    }
}
