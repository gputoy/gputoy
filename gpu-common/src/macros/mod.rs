#[macro_use]
mod actions;
#[macro_use]
mod enums;
#[macro_use]
mod preference_category;
#[macro_use]
mod completions;

#[macro_export]
macro_rules! extract_top_doc_comment_str {
    (
        #[doc = $doc:expr]
        $(#[$($_attr:tt)*])*
    ) => {
        $doc
    };
    ($(#[$($_attr:tt)*])*) => {
        ""
    };
    () => {
        ""
    };
}

#[macro_export]
macro_rules! kebab_case {
    ($t:tt) => {{
        let val: &'static str = stringify!($t);
        ::convert_case::Casing::to_case(&val, ::convert_case::Case::Kebab)
    }};
    (static $t:tt) => {{
        ::lazy_static::lazy_static! {
            static ref __KEBAB_CASED: String = {
                let val: &'static str = stringify!($t);
                ::convert_case::Casing::to_case(&val, ::convert_case::Case::Kebab)
            };
        }
        __KEBAB_CASED.as_str()
    }};
}

#[macro_export]
macro_rules! append_ts {
    (
        $builder:ident,
        $($toks:tt)+
    ) => {
        {
            let val = $crate::gen_ts!($($toks)+);
            $builder
                .append_ts(val);
        }
    }
}
