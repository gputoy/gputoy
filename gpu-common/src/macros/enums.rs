/// An enum which can be completed and parsed.
///
#[macro_export]
macro_rules! parseable_enum {
    (
        $(#[$($attrs:tt)*])*
        enum $enum:ident {
            $(
                $(#[$($prop_attrs:tt)*])*
                [$($pat:expr),+] => $variant:ident,
            )*
        }
    ) => {
        $(#[$($attrs)*])*
        #[derive(Debug, PartialEq, Eq, Clone, Copy)]
        #[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        #[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
        pub enum $enum {
            $(
                $(#[$($prop_attrs)*])*
                $variant,
            )*
        }

        impl ::core::str::FromStr for $enum {
            type Err = $crate::RootError;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                match value {
                    $(
                        $($pat)|+ => Ok(Self::$variant),
                    )*
                    _ => Err(Self::Err::InvalidValue(
                        stringify!($enum),
                        value.to_owned()
                    ))
                }
            }
        }

        impl $crate::describe::Describe<'_> for $enum {
            fn describe(manifest: &mut $crate::describe::Manifest) {
                manifest
                    .with_name($crate::kebab_case!(static $enum))
                    .with_description(
                        $crate::extract_top_doc_comment_str!(
                            $(#[$($attrs)*])*
                        )
                    )
                    .with_completion($crate::completion::CompletionKey::$enum)
                    .finish_arg()
            }
        }

        impl $crate::parse::Parse<'_> for $enum {
            fn parse(args: &mut $crate::parse::ParseArgs) -> Result<Self, $crate::RootError> {
                args.next_arg().and_then(::core::str::FromStr::from_str)
            }
        }

        #[cfg(feature = "bindgen")]
        impl $crate::complete::Complete for $enum {
            fn static_completions() -> Vec<$crate::completion::CompletionEntry> {
                vec![
                    $(
                        {
                            let name = stringify!($variant);
                            let description =
                                $crate::extract_top_doc_comment_str!($(#[$($prop_attrs)*])*);
                            $crate::parseable_enum!(entry name description $variant $($pat),+)
                        }
                    ),*
                ]
                    .into_iter()
                    .flatten()
                    .collect::<::std::vec::Vec<_>>()
            }
        }
    };
    (entry $name:ident $description:ident $_id:ident $($pat:expr),*) => {
        vec![
            $(
                $crate::completion::CompletionEntry::new(
                    $pat,
                    $name,
                    $description,
                ),
            )*
        ]
    };
    (entry $name:ident $description:ident $id:ident) => {
        vec![
            $crate::completion::CompletionEntry::new(
                stringify!($id),
                $name,
                $description,
            )
        ]
    };
}

/// A superset of [`crate::parseable_enum!`] that adds an additional bindgen pass
/// which allows this enum to appear in the preferences UI.
#[macro_export]
macro_rules! config_enum {
    (
        $(#[$($attrs:tt)*])*
        enum $pref_name:ident {
            $(
                $(#[$($prop_attrs:tt)*])*
                [$($pat:expr),+] => $variant:ident,
            )*
        }
    ) => {
        $crate::parseable_enum! {
            $(#[$($attrs)*])*
            enum $pref_name {
                $(
                    $(#[$($prop_attrs)*])*
                    [$($pat),*] => $variant,
                )*
            }
        }
        #[cfg(feature = "bindgen")]
        impl $crate::config_value::ConfigValue for $pref_name {
            fn metadata(
                prefix: &str,
            ) -> $crate::config_value::ConfigValueSchema {
                let name = $crate::kebab_case!($pref_name);
                #[allow(unused_mut)]
                let description = $crate::extract_doc_comment!($(#[$($attrs)*])*)
                    .to_owned();
                let metadata = $crate::config_value::ConfigValueSchema {
                    name,
                    description,
                    path: prefix.to_owned(),
                    class: $crate::config_value::ConfigValueClass::EnumClass(
                        $crate::config_value::EnumClass {
                            variants: vec![
                                $(
                                    $crate::kebab_case!($variant),
                                )*
                            ]
                        }
                    ),
                };

                metadata
            }
            fn keys(prefix: &str) -> Vec<String> {
                vec![prefix.to_owned()]
            }
        }
    }
}
