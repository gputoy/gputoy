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
pub mod schema;
#[cfg(feature = "bindgen")]
pub use bindgen::bindgen;
#[cfg(feature = "bindgen")]
mod bindgen {
    use super::schema::Schema;
    use super::{PreferenceKey, Preferences};
    use crate::completion::CompletionEntry;

    impl crate::complete::Complete for PreferenceKey {
        fn static_completions() -> Vec<CompletionEntry> {
            Preferences::keys()
                .into_iter()
                .map(|key| CompletionEntry::new_single(key, "preferences", "preferences"))
                .collect()
        }
    }

    pub fn bindgen(builder: crate::bindgen::Builder) -> crate::bindgen::Result<()> {
        builder
            .file("defaults.json")
            .containing_default::<Preferences>()?;

        builder
            .file("schema.json")
            .containing_value(&Preferences::schema())?;

        crate::append_ts! {
            builder,
            "export const PREFERENCE_KEYS = ["
            for k in Preferences::keys().iter()
            "    '{k}',"
            "] as const"
            "export type PreferenceKey = typeof PREFERENCE_KEYS[number]"
        };

        Ok(())
    }
}
