use crate::completion::CompletionKey;

use super::parse::ParseArgs;

pub trait Describe<'val> {
    const ARG_COUNT: usize = 1;
    fn describe(manifest: &mut Manifest);
}

impl<'val, P> Describe<'val> for Option<P>
where
    P: Describe<'val>,
{
    fn describe(manifest: &mut Manifest) {
        P::describe(manifest)
    }
}

impl Describe<'_> for String {
    fn describe(manifest: &mut Manifest) {
        manifest
            .with_name("string")
            .with_completion(CompletionKey::Str)
            .finish_arg()
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct ArgDescriptor<'val> {
    pub value: &'val str,
    pub name: &'static str,
    pub description: &'static str,
    pub completion_key: CompletionKey,
}

#[derive(Debug)]
pub struct Manifest<'val> {
    value: &'val str,
    args: Vec<&'val str>,
    arg_descriptors: Vec<ArgDescriptor<'val>>,
    word_index: usize,
    arg_name: Option<&'static str>,
    description: Option<&'static str>,
    completion_key: Option<CompletionKey>,
}

#[derive(Debug)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct CompletionInfo<'cmd> {
    pub arg_descriptors: Vec<ArgDescriptor<'cmd>>,
    pub cursor_word_index: usize,
}

impl<'val> Manifest<'val> {
    pub fn from_value(value: &'val str) -> Self {
        let args = value.split_whitespace().collect::<Vec<_>>();
        let arg_descriptors = Vec::new();

        Self {
            value,
            args,
            arg_descriptors,
            word_index: 0,
            arg_name: None,
            description: None,
            completion_key: None,
        }
    }
    pub fn peek_arg(&self) -> Option<&'val str> {
        self.args.get(self.word_index).copied()
    }
    pub fn to_parse_args(self) -> ParseArgs<'val> {
        ParseArgs::new(self.args, self.arg_descriptors)
    }
    pub fn to_completion_info(self, cursor_char_index: usize) -> CompletionInfo<'val> {
        let cursor_word_index = get_num_words_until_char(self.value, cursor_char_index);
        // assert!(cursor_word_index < self.arg_descriptors.len());
        CompletionInfo {
            arg_descriptors: self.arg_descriptors,
            cursor_word_index,
        }
    }
}

impl Manifest<'_> {
    pub fn with_name_override(&mut self, name: &'static str) -> &mut Self {
        self.arg_name.replace(name);
        self
    }
    pub fn with_name(&mut self, name: &'static str) -> &mut Self {
        self.arg_name.get_or_insert(name);
        self
    }
    pub fn with_description_override(&mut self, description: &'static str) -> &mut Self {
        self.description.replace(description);
        self
    }
    pub fn with_description(&mut self, description: &'static str) -> &mut Self {
        self.description.get_or_insert(description);
        self
    }
    pub fn with_completion_override(&mut self, completion: CompletionKey) -> &mut Self {
        self.completion_key.replace(completion);
        self
    }

    pub fn with_completion(&mut self, completion: CompletionKey) -> &mut Self {
        self.completion_key.get_or_insert(completion);
        self
    }
    pub fn finish_arg(&mut self) {
        // TODO: come up with something better than magic values
        let arg_descriptor = ArgDescriptor {
            value: self.args.get(self.word_index).unwrap_or(&""),
            name: self.arg_name.take().unwrap_or("[UNNAMED]"),
            description: self.description.take().unwrap_or(""),
            completion_key: self.completion_key.take().unwrap_or(CompletionKey::Empty),
        };
        self.arg_descriptors.push(arg_descriptor);
        self.word_index += 1;
    }
}

fn get_num_words_until_char(value: &str, char_pos: usize) -> usize {
    let mut last_whitespace = false;
    let mut cursor_word_pos = 0;
    for char in value.chars().take(char_pos) {
        let is_whitespace = char.is_whitespace();
        if last_whitespace && !is_whitespace {
            cursor_word_pos += 1;
            last_whitespace = false;
        } else if !last_whitespace && is_whitespace {
            last_whitespace = true;
        }
    }
    cursor_word_pos
}
