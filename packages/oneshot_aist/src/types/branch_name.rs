use std::ffi::OsString;
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
