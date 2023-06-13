pub mod actions;
pub mod completion;
pub mod layout;
pub mod preferences;

#[cfg(feature = "bindgen")]
pub fn bindgen(builder: crate::bindgen::Builder) -> crate::bindgen::Result<()> {
    completion::bindgen(builder)?;
    Ok(())
}
