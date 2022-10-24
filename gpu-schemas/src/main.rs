use schemars::{schema::RootSchema, schema_for};

fn write_schema(dir: &std::path::Path, name: &str, schema: &RootSchema) -> std::io::Result<()> {
    let output = serde_json::to_string_pretty(schema).unwrap();
    let output_path = dir.join(format!("{}.json", name));
    std::fs::write(output_path, output)
}

fn main() -> std::io::Result<()> {
    let dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("schemas");
    match std::fs::DirBuilder::new().create(&dir) {
        Err(e) if e.kind() != std::io::ErrorKind::AlreadyExists => return Err(e),
        _ => (),
    }

    let schema = &schema_for!(gpu_common::Project);
    write_schema(&dir, "project", schema)?;

    let schema = &schema_for!(gpu_common::Config);
    write_schema(&dir, "config", schema)?;

    let schema = &schema_for!(gpu_common::realm::ProjectUpsert);
    write_schema(&dir, "project_upsert", schema)?;

    let schema = &schema_for!(gpu_common::realm::ProjectResponse);
    write_schema(&dir, "project_reponse", schema)?;

    let schema = &schema_for!(gpu_common::realm::NewUser);
    write_schema(&dir, "new_user", schema)?;

    let schema = &schema_for!(gpu_common::realm::NewUserResponse);
    write_schema(&dir, "new_user_response", schema)?;

    let schema = &schema_for!(gpu_common::realm::Credentials);
    write_schema(&dir, "credentials", schema)?;

    let schema = &schema_for!(gpu_common::realm::LoginResponse);
    write_schema(&dir, "login_response", schema)?;

    let schema = &schema_for!(gpu_common::realm::UserInfoResponse);
    write_schema(&dir, "user_info_response", schema)?;

    let schema = &schema_for!(gpu_common::Action);
    write_schema(&dir, "action", schema)?;

    println!("Wrote schemas to {}", dir.to_string_lossy());

    Ok(())
}
