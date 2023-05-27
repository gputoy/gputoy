pub mod actions;
pub mod completion;
#[cfg(feature = "bindgen")]
pub mod config_value;
pub mod layout;
pub mod preferences;

#[cfg(feature = "bindgen")]
pub fn bindgen(builder: crate::bindgen::Builder) -> crate::bindgen::Result<()> {
    completion::bindgen(builder)?;
    Ok(())
}
