#[macro_export]
macro_rules! store_for {
    ($store:ident => $proc:ty) => {
        pub struct $store(
            ::slotmap::SecondaryMap<
                $crate::model::ModelKey,
                <$proc as $crate::analyzer::Process>::Result,
            >,
        );

        impl $crate::analyzer::ProcessStore<$proc> for $store {
            fn with_capacity(capacity: usize) -> Self {
                Self(::slotmap::SecondaryMap::with_capacity(capacity))
            }

            fn get(
                &self,
                key: $crate::model::ModelKey,
            ) -> $crate::Result<&<$proc as $crate::analyzer::Process>::Result> {
                self.0.get(key).ok_or($crate::Error::StoreValueVacant(
                    <$proc as $crate::analyzer::Process>::STAGE_VARIANT,
                    key,
                ))
            }

            fn store(
                &mut self,
                key: $crate::model::ModelKey,
                value: <$proc as $crate::analyzer::Process>::Result,
            ) -> Option<<$proc as $crate::analyzer::Process>::Result> {
                self.0.insert(key, value)
            }
        }
    };
}

#[macro_export]
macro_rules! dispatch_store {
    ($($name:ident: $proc:ty,)*) => {
        pub struct DispatchStorage {
            version: ::slotmap::SecondaryMap<$crate::model::ModelKey, $crate::analyzer::ProcessProgress>,
            $(
                $name: <$proc as Process>::Store,
            )*
        }

        impl DispatchStorage {
            pub fn with_capacity(capacity: usize) -> Self {
                Self {
                    version: ::slotmap::SecondaryMap::with_capacity(capacity),
                    $(
                        $name: <$proc as Process>::Store::with_capacity(capacity),
                    )*
                }
            }

        }

        $(
            impl StoreProvider<$proc> for DispatchStorage {
                fn get_store(&self) -> &<$proc as Process>::Store {
                    &self.$name
                }
                fn get_store_mut(&mut self) -> &mut <$proc as Process>::Store {
                    &mut self.$name
                }
            }
        )*
    };
}

#[macro_export]
macro_rules! dispatch {


    ($($field:ident: $proc:ty,)+) => {
        impl Dispatcher {
            pub fn _dispatch_all(
                analyzer: &mut $crate::Analyzer,
            ) -> ($crate::analyzer::DispatchManifest, Vec<Box<dyn $crate::diagnostics::IntoDiagnostic>>) {
                let mut _errors = vec![];
                let mut manifest = ::std::default::Default::default();

                $(
                    let mut _stage_errors = Self::dispatch_process::<$proc>(analyzer, &mut manifest);
                    let fail = _stage_errors.should_fail();
                    _errors.append(&mut _stage_errors.into_inner());
                    if fail {
                        return (manifest, _errors);
                    }
                )*

                (manifest, _errors)
            }
        }
    };
}
