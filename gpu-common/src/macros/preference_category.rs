#[macro_export]
macro_rules! config_category {
    (
        $(#[$($attrs:tt)*])*
        struct $pref_name:ident {
            $(
                $(#[$($prop_attrs:tt)*])*
                $prop_name:ident: $($manual_ty:ty)? $({
                    Self = $class_ty:ty,
                    class = $class:ident,
                    $($class_prop:ident = $class_val:expr,)*
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
                $(#[$($prop_attrs)*])*
                pub $prop_name: $($manual_ty)? $($class_ty)?,
            )*
        }

        #[cfg(feature = "bindgen")]
        #[allow(unused_doc_comments)]
        impl $crate::preferences::schema::Schema for $pref_name {
            fn _schema_impl(
                mut builder: $crate::preferences::schema::Builder
            ) -> $crate::preferences::schema::Builder {
                $(
                    {
                        let name = $crate::to_case!("kebab-case", $prop_name);
                        let description = $crate::extract_doc_comment!($(#[$($prop_attrs)*])*);
                        $(
                            builder = <$manual_ty as $crate::preferences::schema::Schema>::_schema_impl(
                                builder
                                    .push_name(name)
                                    .push_description(description)
                            )
                                .pop();
                        )?
                        $(
                            let class = $crate::preferences::schema::ConfigClass::$class(
                                $crate::preferences::schema::$class::default()
                                    $(.$class_prop($class_val))*
                            );
                            builder = builder
                                .push_name(name)
                                .push_description(description)
                                .push_config_class(class)
                                .pop();
                        )?
                    }
                )*

                builder
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
