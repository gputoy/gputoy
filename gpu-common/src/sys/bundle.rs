#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[cfg(feature = "deserialize")]
use serde::Deserialize;
#[cfg(feature = "serialize")]
use serde::Serialize;
use strum_macros::{AsRefStr, EnumString};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
#[cfg_attr(
    any(feature = "serialize", feature = "deserialize"),
    serde(tag = "type")
)]
pub enum BundleArgs {
    Viewport(ViewportBundleArgs),
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
pub struct ViewportBundleArgs {
    /// Target canvas id
    pub target: String,
}

#[derive(Debug, Clone, PartialEq, Eq, EnumString, AsRefStr)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
#[cfg_attr(
    any(feature = "serialize", feature = "deserialize"),
    serde(rename_all = "camelCase")
)]
#[strum(serialize_all = "camelCase")]
pub enum ViewportBundleResources {
    Surface,
    Mouse,
    Resolution,
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
