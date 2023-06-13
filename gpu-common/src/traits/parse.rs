use crate::describe::ArgDescriptor;
use crate::RootError;

pub trait Parse<'val>: crate::describe::Describe<'val> + Sized {
    type Error: crate::IntoClientError = RootError;

    fn parse(args: &mut ParseArgs<'val>) -> Result<Self, Self::Error>;
}

impl<'val, P> Parse<'val> for Option<P>
where
    P: Parse<'val>,
{
    type Error = P::Error;

    fn parse(args: &mut ParseArgs<'val>) -> Result<Self, Self::Error> {
        if args.has_next() {
            Ok(Some(P::parse(args)?))
        } else {
            Ok(None)
        }
    }
}

impl Parse<'_> for String {
    fn parse(args: &mut ParseArgs<'_>) -> Result<Self, Self::Error> {
        args.next_arg().map(String::from)
    }
}

#[derive(Debug)]
pub struct ParseArgs<'val> {
    pub args: Vec<&'val str>,
    arg_descriptors: Vec<ArgDescriptor<'val>>,
    word_index: usize,
}

impl<'val> ParseArgs<'val> {
    pub fn new(args: Vec<&'val str>, arg_descriptors: Vec<ArgDescriptor<'val>>) -> ParseArgs<'val> {
        Self {
            args,
            arg_descriptors,
            word_index: 0,
        }
    }
    pub fn next_arg(&mut self) -> Result<&str, RootError> {
        let arg_descriptor = self.arg_descriptors.get(self.word_index).ok_or_else(|| {
            RootError::UnknownArgument(self.args.get(self.word_index).unwrap_or(&"").to_string())
        })?;
        let arg = self
            .args
            .get(self.word_index)
            .ok_or_else(|| RootError::MissingArgument(arg_descriptor.name.into()))?;
        self.word_index += 1;
        Ok(*arg)
    }

    pub fn has_next(&self) -> bool {
        self.args.len() < self.word_index + 1
    }
}
