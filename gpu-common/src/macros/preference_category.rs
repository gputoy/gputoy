#[macro_export]
macro_rules! config_category {
    (
        $(#[$($attrs:tt)*])*
        struct $pref_name:ident$(<Toggle: $marker:ty>)? {
            $(
                $(#[$($prop_attrs:tt)*])*
                $prop:ident: $($manual_ty:ty)? $({
                    Self = $class_ty:ty,
                    class = $class:ident,
                    $($class_prop:ident = $class_val:literal,)*
                })?,
            )*
        }
    ) => {
        $(#[$($attrs)*])*
        #[derive(::core::fmt::Debug)]
        #[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        #[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
        pub struct $pref_name {
            $(
                pub enabled: $marker,
            )?
            $(
                $(#[$($prop_attrs)*])*
                pub $prop: $($manual_ty)? $($class_ty)?,
            )*
        }

        #[cfg(feature = "bindgen")]
        #[allow(unused_doc_comments)]
        impl $crate::config_value::ConfigValue for $pref_name {
            fn metadata(
                prefix: &str,
            ) -> $crate::config_value::ConfigValueSchema {
                let mut map = ::std::collections::HashMap::new();
                let prefix = if prefix.is_empty() {
                    "".to_owned()
                } else {
                    format!("{}.", prefix)
                };
                let name = $crate::kebab_case!($pref_name);
                #[allow(unused_mut)]
                let description = $crate::extract_doc_comment!($(#[$($attrs)*])*);
                $(
                    let prop_name = $crate::kebab_case!($prop);
                    let full_path = format!("{}{}", prefix, prop_name);
                    #[allow(unused_variables)]
                    let prop_description = $crate::extract_doc_comment!($(#[$($prop_attrs)*])*);
                    $(
                        let prop_metadata = <$manual_ty as $crate::config_value::ConfigValue>::metadata(&full_path);
                        map.insert(prop_name.clone(), prop_metadata.clone());
                    )?
                    $(
                        let class = $crate::config_value::$class::default()
                        $(
                            .$class_prop($class_val)
                        )*;

                        let prop_metadata = $crate::config_value::ConfigValueSchema {
                            name: prop_name.clone(),
                            description: prop_description.to_owned(),
                            path: full_path.clone(),
                            class: $crate::workspace::config_value::ConfigValueClass::$class(class),
                        };
                        map.insert(prop_name, prop_metadata.clone());
                    )?
                )*
                $crate::config_value::ConfigValueSchema {
                    name,
                    description: description.to_owned(),
                    path: "".to_owned(),
                    class: $crate::maybe_toggle!($($marker)?)(
                        $crate::config_value::CategoryClass {inner: map}
                    ),
                }
            }
            fn keys(prefix: &str) -> Vec<String> {
                #[allow(unused_mut, unused_variables)]
                let mut keys = vec![];
                let prefix = if prefix.is_empty() {
                    "".to_owned()
                } else {
                    format!("{}.", prefix)
                };

                $(
                    let prop_name = format!("{}{}", prefix, $crate::kebab_case!($prop));
                    $(
                        let subkeys = <$manual_ty as $crate::config_value::ConfigValue>::keys(&prop_name);
                        keys.extend(subkeys);
                    )?
                    $(
                        #[doc = stringify!($class_ty)]
                        keys.push(prop_name);
                    )?
                )*
                keys
            }
        }
    }
}

#[macro_export]
macro_rules! maybe_toggle {
    ($marker:ty) => {
        $crate::config_value::ConfigValueClass::ToggledCategoryClass
    };
    () => {
        $crate::config_value::ConfigValueClass::CategoryClass
    };
}
