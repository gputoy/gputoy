#[macro_export]
macro_rules! actions {
    (
        $(
            $(#[$($attr:tt)*])*
            [$($pat:expr),*] => $name:ident$(($arg:ty))?,
        )*
    ) => {

        $crate::parseable_enum! {
            enum ActionKey {
                $(
                    $(#[$($attr)*])*
                    [$($pat),*] => $name,
                )*
            }
        }

        #[derive(Debug, Clone, PartialEq, Eq)]
        #[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        #[cfg_attr(feature = "serde", serde(rename_all = "camelCase", tag = "ty", content = "c"))]
        pub enum Action {
            $(
                $(#[$($attr)*])*
                $name$(($arg))?,
            )*
        }

        impl $crate::describe::Describe<'_> for Action {
            fn describe(manifest: &mut $crate::describe::Manifest<'_>) {
                use ::core::str::FromStr;
                // It is important that the first arg is peeked BEFORE describing the ActionKey.
                let peek_arg = manifest.peek_arg();
                ActionKey::describe(manifest);
                let Some(action_key) = peek_arg.and_then(|arg| ActionKey::from_str(arg).ok()) else {
                    return
                };
                match action_key {
                    $(
                        $(
                            ActionKey::$name => {
                                <$arg as $crate::describe::Describe>::describe(manifest);
                            }
                        )?
                    )*
                    _ => {}
                }
            }
        }

        impl $crate::parse::Parse<'_> for Action {
            fn parse(args: &mut $crate::parse::ParseArgs<'_>) -> Result<Self, Self::Error> {
                let action_key = ActionKey::parse(args)?;
                let action = match action_key {
                    $(
                        ActionKey::$name => Self::$name$(
                            (<$arg as $crate::parse::Parse>::parse(args)?)
                        )?,
                    )*
                };
                Ok(action)
            }
        }
    }
}
