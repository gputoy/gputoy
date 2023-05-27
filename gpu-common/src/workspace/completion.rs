#[cfg(feature = "bindgen")]
#[derive(Debug, Clone, schemars::JsonSchema, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CompletionEntry {
    insert_text: String,
    name: String,
    description: String,
}

#[cfg(feature = "bindgen")]
impl CompletionEntry {
    pub fn new<T, N, D>(insert_text: T, name: N, description: D) -> Self
    where
        T: Into<String>,
        N: Into<String>,
        D: Into<String>,
    {
        Self {
            insert_text: insert_text.into(),
            name: name.into(),
            description: description.into(),
        }
    }
}

crate::completions! {
    /// Complete actions
    ///
    /// Generated statically from bindgen.
    ActionKey: crate::ActionKey,

    /// Complete strings
    ///
    /// At the moment does nothing, but could be useful to distinguish in the future.
    Str,

    /// Complete file paths.
    ///
    /// Generated at runtime from svelte file store
    FilePath,

    /// Complete directory paths
    ///
    /// Generated at runtime from svelte file store.
    Path,

    /// Complete resource paths (i.e. 'viewport::texture')
    ///
    /// Generated from @TODO
    Resource,

    /// Complete config key paths (i.e. 'editor.vim_mode.enabled')
    ///
    /// Generated statically from bindgen.
    PreferenceKey: crate::preferences::PreferenceKey,

    /// Complete ui region (i.e. 'terminal' or 'editor')
    ///
    /// Generated statically from bindgen.
    Region: crate::Region,

    /// Complete key (i.e. 'C-a')
    ///
    /// At the moment does nothing, but could be useful to distinguish in the future.
    Key,

    // Auto-generated enum variants
    AutoIndent: crate::preferences::editor::AutoIndent,
    CursorBlinking: crate::preferences::editor::CursorBlinking,
    CursorStyle: crate::preferences::editor::CursorStyle,
    LineNumbers: crate::preferences::editor::LineNumbers,
    LogLevel: crate::preferences::console::LogLevel,
    ShowCompletions: crate::preferences::console::ShowCompletions,
    UiSpeed: crate::preferences::workspace::UiSpeed,
}

#[cfg(feature = "bindgen")]
pub fn bindgen(builder: crate::bindgen::Builder) -> crate::bindgen::Result<()> {
    builder
        .file("completions.json")
        .containing_value(&self::generate_static_completions())?;
    builder.append_ts(generate_key_index_ts());
    Ok(())
}
