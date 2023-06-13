use gpu_common::{describe::CompletionInfo, method, Action, FilePath, Path, RootError};

#[method]
pub fn action(cmd: String) -> Result<Action, RootError> {
    gpu_common::runtime::Parser::parse(&cmd)
}

#[method]
pub fn action_completion(command: String, cursor_char_pos: usize) -> CompletionInfo {
    gpu_common::runtime::Completions::arg_info::<Action>(&command, cursor_char_pos)
}

#[method]
pub fn file_path(path: String) -> Result<FilePath, RootError> {
    gpu_common::FilePath::try_from(path)
}

#[method]
pub fn path(path: String) -> Result<Path, RootError> {
    gpu_common::Path::try_from(path)
}
