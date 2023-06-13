#[macro_export]
macro_rules! completions {
    (
        $(
            $(#[$($attrs:tt)*])*
            $ty_name:ident$(: $ty_path:path)?,
        )*
    ) => {

        #[derive(Debug, Copy, Clone, Eq, PartialEq)]
        #[cfg_attr(feature = "bindgen", derive(strum_macros::EnumIter, Hash))]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, ::serde::Deserialize))]
        #[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
        pub enum CompletionKey {
            /// No completions.
            Empty = 0,
            $(
                $(#[$($attrs)*])*
                $ty_name,
            )*
        }

        #[cfg(feature = "bindgen")]
        fn generate_static_completions() -> Vec<Vec<$crate::completion::CompletionEntry>> {
            use ::strum::IntoEnumIterator;
            CompletionKey::iter()
                .map(|ty| {
                    match ty {
                        CompletionKey::Empty => vec![],
                        $(
                            CompletionKey::$ty_name => {
                                $crate::completions!(maybe $($ty_path)?)
                            }
                        )*
                    }
                })
                .collect::<Vec<_>>()
        }

        #[cfg(feature = "bindgen")]
        fn generate_key_index_ts() -> String {
            let idents = [
                $(
                    stringify!($ty_name),
                )*
            ];
            $crate::gen_ts! {
                "export enum CompletionIndex {{"
                    "\tEmpty = 0,"
                for ident in idents.iter()
                    "\t{ident},"
                "}}"
                "export const COMPLETION_KEY_TO_INDEX: Record<CompletionKey, CompletionIndex> = {{"
                    "\t'Empty': CompletionIndex.Empty,"
                for ident in idents.iter()
                    "\t'{ident}': CompletionIndex.{ident},"
                "}} as const"
            }
        }
    };
    (maybe $ty_path:path) => {
        <$ty_path as $crate::complete::Complete>::static_completions()
    };
    (maybe) => {
        vec![]
    };
}
