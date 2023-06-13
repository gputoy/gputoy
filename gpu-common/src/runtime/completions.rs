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
        let completion_key = if let Some(word_index) = completion_info.cursor_word_index {
            completion_info
                .arg_descriptors
                .get(word_index)
                .expect(&format!(
                    "Cursor index bounds: '{command}'@{cursor_position}"
                ))
                .completion_key
        } else {
            Empty
        };

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
        test_completion_key(" p ", 0, Empty);
        test_completion_key(" p ", 1, ActionKey);
        test_completion_key(" p ", 2, ActionKey);
        test_completion_key(" p  ", 3, Empty);
        test_completion_key(" p k", 3, Empty);
    }

    #[test]
    fn test_one_arg_actions() {
        test_completion_key("show editor", 0, ActionKey);
        test_completion_key(" show editor", 0, Empty);
        test_completion_key(" show editor", 1, ActionKey);
        test_completion_key(" show editor", 5, ActionKey);
        test_completion_key(" show editor", 6, Region);
        test_completion_key(" show  editor", 6, Empty);
        test_completion_key(" show  editor", 7, Region);
        test_completion_key(" show  editor", 12, Region);
        test_completion_key(" show  editor ", 13, Region);
        test_completion_key(" show  editor  ", 14, Empty);
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
        test_completion_key("bind C-a toggle editor", 4, ActionKey);
        test_completion_key("bind C-a toggle editor", 5, Key);
        test_completion_key("bind C-a toggle editor", 8, Key);
        test_completion_key("bind C-a toggle editor", 9, ActionKey);
        test_completion_key("bind C-a toggle editor", 15, ActionKey);
        test_completion_key("bind C-a toggle editor", 16, Region);
        test_completion_key("bind C-a toggle editor", 21, Region);
        test_completion_key(" bind C-a toggle editor", 0, Empty);
        test_completion_key(" bind C-a toggle editor", 1, ActionKey);
        test_completion_key("bind C-a toggle editor ", 22, Region);
        test_completion_key("bind C-a toggle editor ", 23, Empty);
    }
}
