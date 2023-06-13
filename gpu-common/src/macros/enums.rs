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
        #[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
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
                    .with_name($crate::to_case!("kebab-case", $enum))
                    .with_description(
                        $crate::extract_doc_comment!($(#[$($attrs)*])*)
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
                                $crate::extract_doc_comment!($(#[$($prop_attrs)*])*);
                            $crate::parseable_enum!(entry name description $variant $($pat),+)
                        }
                    ),*
                ]
            }
        }
    };
    (entry $snippet_text:ident $description:ident $_id:ident $($pat:expr),*) => {
        $crate::completion::CompletionEntry::new_aliased(
            [
                $(
                    $pat,
                )+
            ],
            $snippet_text,
            $description,
        )
    };
    (entry $snippet_text:ident $description:ident $id:ident) => {
        vec![
            $crate::completion::CompletionEntry::new(
                stringify!($id),
                $snippet_text,
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
        impl $crate::preferences::schema::Schema for $pref_name {
            fn _schema_impl(
                builder: $crate::preferences::schema::Builder
            ) -> $crate::preferences::schema::Builder {
                let description = $crate::extract_doc_comment!($(#[$($attrs)*])*);
                let class = $crate::preferences::schema::ConfigClass::Enum(
                    $crate::preferences::schema::Enum {
                        variants: vec![$($crate::to_case!("kebab-case", $variant).to_owned(),)*],
                    }
                );
                builder
                    .push_description(description)
                    .push_config_class(class)
            }
        }
    }
}
