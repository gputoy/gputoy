use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone, Copy)]
#[wasm_bindgen]
pub struct Position {
    #[wasm_bindgen(js_name = "lineNumber")]
    pub line_number: u32,
    pub column: u32,
}

#[wasm_bindgen]
impl Position {
    #[wasm_bindgen(constructor)]
    pub fn new(column: u32, line_number: u32) -> Self {
        Self {
            column,
            line_number,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompletionItem {
    /// The label of this completion item. By default
    /// this is also the text that is inserted when selecting
    /// this completion.
    pub label: String,
    /// The kind of this completion item. Based on the kind
    /// an icon is chosen by the editor.
    pub kind: CompletionItemKind,
    /// A modifier to the `kind` which affect how the item
    /// is rendered, e.g. Deprecated is rendered with a strikeout
    pub tags: Option<CompletionItemTag>,
    pub detail: Option<String>,
    pub documentation: Option<String>,
    pub insert_text: String,
    /// Additional rules (as bitmask) that should be applied when inserting
    /// this completion.
    pub insert_text_rules: Option<CompletionItemInsertTextRule>,
    /// A range of text that should be replaced by this completion item.
    pub range: Option<()>,
}

impl Default for CompletionItem {
    fn default() -> Self {
        Self {
            label: String::new(),
            kind: CompletionItemKind::Text,
            tags: None,
            detail: None,
            documentation: None,
            insert_text: String::new(),
            insert_text_rules: None,
            range: None,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum CompletionItemKind {
    Method = 0,
    Function = 1,
    Constructor = 2,
    Field = 3,
    Variable = 4,
    Class = 5,
    Struct = 6,
    Interface = 7,
    Module = 8,
    Property = 9,
    Event = 10,
    Operator = 11,
    Unit = 12,
    Value = 13,
    Constant = 14,
    Enum = 15,
    EnumMember = 16,
    Keyword = 17,
    Text = 18,
    Color = 19,
    File = 20,
    Reference = 21,
    Customcolor = 22,
    Folder = 23,
    TypeParameter = 24,
    User = 25,
    Issue = 26,
    Snippet = 27,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum CompletionItemTag {
    Deprecated = 1,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum CompletionItemInsertTextRule {
    KeepWhitespace = 1,
    InsertAsSnippet = 4,
}
