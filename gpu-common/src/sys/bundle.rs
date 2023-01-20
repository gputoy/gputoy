#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, EnumString};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum BundleArgs {
    Viewport(ViewportBundleArgs),
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ViewportBundleArgs {
    /// Target canvas id
    pub target: String,
}

#[derive(Debug, Clone, PartialEq, Eq, EnumString, AsRefStr)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[strum(serialize_all = "camelCase")]
pub enum ViewportBundleResources {
    Surface,
    Mouse,
    Resolution,
}

#[derive(Debug, Clone, PartialEq, Eq, EnumString, AsRefStr)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[strum(serialize_all = "camelCase")]
pub enum SystemBundleResources {
    Buffer,
}

#[cfg(test)]
mod tests {
    use super::ViewportBundleResources;

    #[test]
    fn test_bundle_args_strings() {
        assert_eq!(ViewportBundleResources::Surface.as_ref(), "surface");

        assert_eq!(
            ViewportBundleResources::try_from("surface").unwrap(),
            ViewportBundleResources::Surface
        );
    }
}
