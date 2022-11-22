#[cfg(all(feature = "serialize", feature = "deserialize", feature = "schema"))]
macro_rules! make_schemas {
    ($dir:ident, $($name:ident),*) => {

        fn write_schema(dir: &std::path::Path, name: &str, schema: &schemars::schema::RootSchema) -> std::io::Result<()> {
            let output = serde_json::to_string_pretty(schema).unwrap();
            let output_path = dir.join(format!("{}.json", name));
             std::fs::write(output_path, output)
        }
       $(
            let schema = &schemars::schema_for!($name);
            write_schema(&$dir, stringify!($name), schema)?;
       )*
    };
}

/// Writes JSON schemas to `../schemas`.
/// Used by cargo-make command `cargo make types` (aliased to `cargo types`) which also
/// runs node script `front/generate_common_types.js` to generate typescript types.
#[cfg(all(feature = "serialize", feature = "deserialize", feature = "schema"))]
fn main() -> std::io::Result<()> {
    use gpu_common::{realm::*, *};
    let dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("schemas");
    match std::fs::DirBuilder::new().create(&dir) {
        Err(e) if e.kind() != std::io::ErrorKind::AlreadyExists => return Err(e),
        _ => (),
    }

    make_schemas!(
        dir,
        Project,
        Config,
        ProjectUpsert,
        ProjectResponse,
        NewUser,
        NewUserResponse,
        Credentials,
        LoginResponse,
        UserInfoResponse,
        UpdateUserInfoArgs,
        Action,
        PrebuildResult
    );

    println!("Wrote schemas to {}", dir.to_string_lossy());

    Ok(())
}

#[cfg(not(all(feature = "serialize", feature = "deserialize", feature = "schema")))]
fn main() {
    println!(
        "You probably meant to run this with features 'serialize', 'deserialize', and 'schema'."
    );
    println!("Aborting.");
}
