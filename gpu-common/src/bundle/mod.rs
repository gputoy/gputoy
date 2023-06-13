#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub mod system;
pub mod viewport;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum BundleArgs {
    Viewport(viewport::Args),
}

pub trait Bundle {
    const TYPE_DECL: &'static str;
}

#[cfg(test)]
mod tests {
    use super::viewport::Resources;

    #[test]
    fn test_bundle_args_strings() {
        assert_eq!(Resources::Surface.as_ref(), "surface");
        assert_eq!(Resources::try_from("surface").unwrap(), Resources::Surface);
    }
}
