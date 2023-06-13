#[macro_use]
mod actions;
#[macro_use]
mod enums;
#[macro_use]
mod preference_category;
#[macro_use]
mod completions;

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
