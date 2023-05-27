use crate::describe::{CompletionInfo, Describe, Manifest};

#[derive(Copy, Clone)]
pub struct Completions;

impl Completions {
    pub fn arg_info<'val, D>(value: &'val str, cursor_char_index: usize) -> CompletionInfo<'val>
    where
        D: Describe<'val>,
    {
        let mut manifest = Manifest::from_value(value);
        D::describe(&mut manifest);
        manifest.to_completion_info(cursor_char_index)
    }
}

#[cfg(test)]
mod tests {
    use crate::completion::CompletionKey::{self, *};
    use crate::Action;

    use super::Completions;

    fn test_completion_key(command: &str, cursor_position: usize, expected_key: CompletionKey) {
        let completion_info = Completions::arg_info::<Action>(command, cursor_position);
        let completion_key =
            completion_info.arg_descriptors[completion_info.cursor_word_index].completion_key;
        assert_eq!(
            completion_key,
            expected_key,
            "Failed test on:\n{command}\n{gap}|\n{gap}| cursor position\n",
            gap = (0..cursor_position).map(|_| " ").collect::<String>()
        );
    }

    #[test]
    fn test_simple_actions() {
        test_completion_key("p", 0, ActionKey);
        test_completion_key("save", 0, ActionKey);
        test_completion_key("publish", 0, ActionKey);
        test_completion_key("    p    ", 0, ActionKey);
        test_completion_key("w        ", 0, ActionKey);
        test_completion_key("    publish", 0, ActionKey);
    }

    #[test]
    fn test_one_arg_actions() {
        test_completion_key("show editor", 6, Region);
        test_completion_key("file /run.json", 8, FilePath);
        test_completion_key("open //run.json", 14, FilePath);
    }

    #[test]
    fn test_complex_actions() {
        test_completion_key("cp /run.json /run-copy.json", 12, FilePath);
        test_completion_key("cp /run.json /run-copy.json", 15, FilePath);
        test_completion_key("mv /run.json /some/other/path/run.json", 12, FilePath);
        test_completion_key("mv /cool-shaders /crappy-shaders", 15, Path);
    }

    #[test]
    fn test_completion_boundaries() {
        test_completion_key("bind C-a toggle editor", 0, ActionKey);
        test_completion_key("bind C-a toggle editor", 5, ActionKey);
        test_completion_key("bind C-a toggle editor", 6, Key);
        test_completion_key("bind C-a toggle editor", 9, Key);
        test_completion_key("bind C-a toggle editor", 10, ActionKey);
        test_completion_key("bind C-a toggle editor", 16, ActionKey);
        test_completion_key("bind C-a toggle editor", 17, Region);
        test_completion_key("bind C-a toggle editor", 22, Region);
    }
}
