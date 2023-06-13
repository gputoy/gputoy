#![feature(exitcode_exit_method)]

fn main() {
    pretty_env_logger::init();
    let config_path = std::env::var("BINDGEN_CONFIG_PATH").expect("BINDGEN_CONFIG_PATH");
    let config = std::fs::read_to_string(config_path).expect("Valid CONFIG_PATH");
    let config = toml::from_str(&config).expect("Toml config parse");
    match gpu_common::bindgen(config) {
        Ok(_) => {
            log::info!("Bindgen finished");
        }
        Err(err) => {
            log::error!("Bindgen failed: {err}");
            std::process::ExitCode::FAILURE.exit_process();
        }
    }
}
