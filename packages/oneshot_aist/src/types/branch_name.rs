use std::ffi::{OsStr, OsString};
use subtype::subtype_string;

subtype_string!(
    pub struct BranchName(String);
);

impl BranchName {}

impl From<BranchName> for OsString {
    fn from(val: BranchName) -> Self {
        val.0.into()
    }
}

impl AsRef<OsStr> for BranchName {
    fn as_ref(&self) -> &OsStr {
        self.0.as_ref()
    }
}
