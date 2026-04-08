use std::collections::HashMap;

use semver::Version;
use syn::Field;

use crate::compat::Kind;

pub(crate) struct ToCompatStruct<'a> {
    pub(crate) semver_compat: HashMap<Version, HashMap<&'a Field, Kind>>,
}
