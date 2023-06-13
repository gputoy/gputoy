/// A completion.
///
/// Completions are generated from gpu-common types as part of the bindgen,
/// but also at runtime for completions that cannot be known statically
/// (i.e. file paths, resources).
#[cfg(feature = "bindgen")]
#[derive(Debug, Clone, schemars::JsonSchema, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CompletionEntry {
    /// A list of aliases this completion will match on.
    /// It will always insert the first alias in the vector.
    matches: Vec<String>,
    /// The snippet information on the right side of a completion.
    snippet_text: String,
    /// A long description of this completion.
    description: String,
}

#[cfg(feature = "bindgen")]
impl CompletionEntry {
    pub fn new_single<M, N, D>(insert_match: M, snippet_text: N, description: D) -> Self
    where
        M: Into<String>,
        N: Into<String>,
        D: Into<String>,
    {
        Self {
            matches: vec![insert_match.into()],
            snippet_text: snippet_text.into(),
            description: description.into(),
        }
    }
    pub fn new_aliased<M, S, D>(matches: M, snippet_text: S, description: D) -> Self
    where
        M: IntoIterator,
        M::Item: Into<String>,
        S: Into<String>,
        D: Into<String>,
    {
        Self {
            matches: matches.into_iter().map(Into::into).collect(),
            snippet_text: snippet_text.into(),
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

    /// Store key
    ///
    /// Generated at runtime on store initialization
    StoreKey,

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
