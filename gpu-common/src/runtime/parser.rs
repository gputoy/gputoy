use crate::{describe::Manifest, parse::Parse};

#[derive(Copy, Clone)]
pub struct Parser;

impl Parser {
    pub fn parse<'val, P>(value: &'val str) -> Result<P, P::Error>
    where
        P: Parse<'val>,
    {
        let mut manifest = Manifest::from_value(value);
        P::describe(&mut manifest);
        P::parse(&mut manifest.to_parse_args())
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::args::{BindKey, CopyMove, Key};
    use crate::Region;
    use crate::{Action, FilePath};

    use super::Parser;

    #[test]
    fn test_simple_actions() {
        assert_eq!(Parser::parse("p"), Ok(Action::PlayPause));
        assert_eq!(Parser::parse("save"), Ok(Action::SaveFile));
        assert_eq!(Parser::parse("publish"), Ok(Action::Publish));

        assert_eq!(Parser::parse("   p  "), Ok(Action::PlayPause));
        assert_eq!(Parser::parse("w         "), Ok(Action::SaveFile));
        assert_eq!(Parser::parse("      publish"), Ok(Action::Publish));

        assert_eq!(
            Parser::parse::<Action>("not-an-action"),
            Err(crate::RootError::InvalidValue(
                "ActionKey",
                "not-an-action".into()
            ))
        );
    }

    #[test]
    fn test_one_arg_actions() {
        assert_eq!(
            Parser::parse("show editor"),
            Ok(Action::Show(Region::EditorPane))
        );
        assert_eq!(
            Parser::parse("open /run.json"),
            Ok(Action::OpenTab(FilePath::try_from("/run.json").unwrap()))
        );

        assert_eq!(
            Parser::parse::<Action>("open //run.json"),
            Err(crate::RootError::InvalidFilePath("//run.json".into()))
        );
    }

    #[test]
    fn test_complex_actions() {
        assert_eq!(
            Parser::parse("cp /run.json /run-copy.json"),
            Ok(Action::Copy(CopyMove {
                src: crate::Path::try_from("/run.json").unwrap(),
                dest: crate::Path::try_from("/run-copy.json").unwrap(),
                is_dir: false,
            }))
        );
        assert_eq!(
            Parser::parse("mv /run.json /some/other/path/run.json"),
            Ok(Action::Move(CopyMove {
                src: crate::Path::try_from("/run.json").unwrap(),
                dest: crate::Path::try_from("/some/other/path/run.json").unwrap(),
                is_dir: false,
            }))
        );
        assert_eq!(
            Parser::parse("mv /cool-shaders /crappy-shaders"),
            Ok(Action::Move(CopyMove {
                src: crate::Path::try_from("/cool-shaders").unwrap(),
                dest: crate::Path::try_from("/crappy-shaders").unwrap(),
                is_dir: true,
            }))
        );
        assert_eq!(
            Parser::parse::<Action>("mv /a-file.txt /some-dir"),
            Err(crate::RootError::InvalidCopyMove)
        );
    }

    #[test]
    fn test_actions_within_actions() {
        assert_eq!(
            Parser::parse("bind C-a toggle editor"),
            Ok(Action::BindKey(BindKey {
                command: Box::new(Action::ToggleUi(Region::EditorPane)),
                key: Key::from_str("C-a").unwrap()
            }))
        );

        assert_eq!(
            Parser::parse("bind S-A-r run /cool-runner.json"),
            Ok(Action::BindKey(BindKey {
                command: Box::new(Action::SetRunner(
                    crate::FilePath::try_from("/cool-runner.json").unwrap()
                )),
                key: Key::from_str("S-A-r").unwrap()
            }))
        );
    }
}
