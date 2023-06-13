pub mod args;

use crate::{layout::Region, FilePath, Path};

use self::args::StoreKey;

crate::actions! {
    // Ui
    /// Clears the console
    ["clear", "clr"] => Clear,
    /// Shows a region in the workspace
    ["show"] => Show(Region),
    /// Hides a region in the workspace
    ["hide"] => Hide(Region),
    /// Togges a region in the workspace
    ["toggle"] => ToggleUi(Region),
    /// Toggles all panes open and closed
    ["theater"] => ToggleAllPanes,

    // Editor
    /// Open document in editor
    ["open"] => OpenTab(FilePath),
    /// Closes document in editor
    ["close"] => CloseTab,
    /// Next tab in editor
    ["next-tab", "nt"] => NextTab,
    /// Previous tab in editor
    ["prev-tab", "pt"] => PrevTab,

    // File
    /// Creates new file
    ["touch", "file"] => NewFile(FilePath),
    /// Create new dir
    ["mkdir", "dir"] => NewDir(Path),
    /// Move file
    ["move", "mv"] => Move(args::CopyMove),
    /// Copy file
    ["copy", "cp"] => Copy(args::CopyMove),
    /// Delete file
    ["delete", "rm"] => Delete(args::Delete),
    /// Save current file
    ["save", "w"] => SaveFile,
    /// Save all files
    ["save-all", "wa"] => SaveAllFiles,

    // Project management
    /// Creates new project
    ["new-project"] => NewProject(Option<String>),
    /// Commit project to remote
    ["commit"] => Commit,
    /// Fork project
    ["fork"] => Fork(Option<String>),
    /// Publish project
    ["publish"] => Publish,

    // System
    /// Sets runner json file
    ["run"] => SetRunner(FilePath),
    /// Play/Pause the project
    ["play", "pause", "p"] => PlayPause,
    /// Resets project to default state
    ["reset", "r"] => Reset,
    /// Build project
    ["build"] => Build,
    /// Bind key
    ["bind"] => BindKey(args::BindKey),
    /// Dump a store's contents
    ["dump"] => Dump(StoreKey),
    /// Close project
    ["exit", "quit"] => Exit,
}
