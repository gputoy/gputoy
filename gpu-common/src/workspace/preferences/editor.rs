crate::config_category! {
    /// User editor preferences
    struct Editor {
        auto_indent: AutoIndent,
        line_numbers: LineNumbers,
        guides: Guides,
        /// Font to use in the editor.
        ///
        /// Comma-delimited list like "FiraMono,mono"
        font: {
            Self = String,
            class = Str,
        },
        /// Font size of code in the editor.
        font_size: {
            Self = u8,
            class = Int,
            min = 5,
            max = 64,
            postfix = "px",
        },
        /// Render font ligatures in the code editor.
        font_ligatures: bool,
        /// Animate smooth scrolling in the editor.
        smooth_scrolling: bool,
        /// Enable smooth caret movement animations.
        smooth_caret: bool,
        cursor_style: CursorStyle,
        cursor_blinking: CursorBlinking,
        /// Whether to wrap lines in the code editor.
        word_wrap: bool,
        /// Enable scrolling to go one screen size past the last line.
        scroll_beyond_last_line: bool,
        /// Whether to show a minimap in the top-right of the code editor.
        minimap: bool,
        vim_mode: VimMode,
    }
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            font_size: 11,
            font: "mono".to_owned(),
            font_ligatures: true,
            minimap: false,
            smooth_scrolling: false,
            smooth_caret: false,
            scroll_beyond_last_line: false,
            word_wrap: false,
            cursor_style: Default::default(),
            guides: Default::default(),
            cursor_blinking: Default::default(),
            auto_indent: Default::default(),
            line_numbers: Default::default(),
            vim_mode: Default::default(),
        }
    }
}

crate::config_category! {
    /// Enable vim movements in code editor.
    #[derive(Default)]
    struct VimMode {
        enabled: {
            Self = bool,
            class = Bool,
        },
    }
}

crate::config_category! {
    /// Visual guides in the code editor
    struct Guides {
        /// Enable rendering of bracket pair guides.
        bracket_pairs: bool,
        /// Enable rendering of vertical bracket pair guides.
        bracket_pairs_horizontal: bool,
        /// Enable highlighting of the active bracket pair.
        highlight_active_bracket_pair: bool,
        /// Enable rendering of indent guides.
        indentation: bool,
        /// Enable highlighting of the active indent guide.
        highlight_active_indentation: bool,
   }
}

impl Default for Guides {
    fn default() -> Self {
        Self {
            bracket_pairs: true,
            bracket_pairs_horizontal: true,
            highlight_active_bracket_pair: true,
            indentation: true,
            highlight_active_indentation: true,
        }
    }
}

crate::config_enum! {
    /// How line numbers appear in the code editor.
    #[derive(Default)]
    enum LineNumbers {
        /// Show absolute line numbers
        #[default]
        ["on", "true"] => On,
        /// Show every 5 line numbers
        ["interval", "i"] => Interval,
        /// Show relative line numbers
        ["relative", "r"] => Relative,
        /// Hide line numbers
        ["off", "false"] => Off,
    }
}

crate::config_enum! {
    /// Style of the cursor in the code editor.
    #[derive(Default)]
    enum CursorStyle {
        #[default]
        ["line"] => Line,
        ["line-thin"] => LineThin,
        ["block"] => Block,
        ["block-outline"] => BlockOutline,
        ["underline"] => Underline,
        ["unerline-thin"] => UnderlineThin,
    }
}

crate::config_enum! {
    /// Controls whether the editor should automatically
    /// adjust the indentation when users type, paste, move or indent lines.
    #[derive(Default)]
    enum AutoIndent {
        ["none", "off", "false"] => None,
        ["keep"] => Keep,
        ["brackets"] => Brackets,
        #[default]
        ["advanced", "on"] => Advanced,
        ["full"] => Full,
    }
}

crate::config_enum! {
    /// Controls the cursor animation style in the code editor.
    #[derive(Default)]
    enum CursorBlinking {
        #[default]
        ["blink"] => Blink,
        ["smooth"] => Smooth,
        ["phase"] => Phase,
        ["expand"] => Expand,
        ["solid"] => Solid,
    }
}
