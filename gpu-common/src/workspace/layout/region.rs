crate::parseable_enum! {
    /// A UI region within the workspace
    enum Region {
        /// Left pane with project information
        ["project", "p"] => ProjectPane,
        /// Bottom pane with various controls
        ["control", "c"] => ControlPane,
        /// Right pane with code editor
        ["editor", "e"] => EditorPane,
        /// Gear icon in top right
        ["prefs", "preferences"] => Preferences,
        /// Hidden debug pane
        ["debug", "d"] => Debug,
        /// Terminal
        ["terminal", "t", "console", "cmd"] => Terminal,
        /// User information
        ["user", "u"] => User,
    }
}
