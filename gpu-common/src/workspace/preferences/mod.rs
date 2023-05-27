pub mod console;
pub mod editor;
pub mod workspace;

crate::config_category! {
    /// User preferences
    #[derive(Default)]
    struct Preferences {
        /// Workspace preferences
        workspace: workspace::Workspace,
        /// Code editor preferences
        editor: editor::Editor,
        /// Console preferences
        console: console::Console,
    }
}

pub struct PreferenceKey;

#[cfg(feature = "bindgen")]
impl crate::complete::Complete for PreferenceKey {
    fn static_completions() -> Vec<crate::completion::CompletionEntry> {
        use crate::{completion::CompletionEntry, config_value::ConfigValue};

        Preferences::keys("")
            .into_iter()
            .map(|key| CompletionEntry::new(&key, &key, "preferences"))
            .collect()
    }
}

#[cfg(feature = "bindgen")]
pub fn bindgen(builder: crate::bindgen::Builder) -> crate::bindgen::Result<()> {
    use crate::config_value::ConfigValue;

    builder
        .file("defaults.json")
        .containing_default::<Preferences>()?;

    builder
        .file("ui-tree.json")
        .containing_value(&Preferences::metadata(""))?;

    crate::append_ts! {
        builder,
        "export const PREFERENCE_KEYS = ["
        for k in Preferences::keys("").iter()
        "    '{k}',"
        "] as const"
        "export type PreferenceKey = typeof PREFERENCE_KEYS[number]"
    };

    Ok(())
}
