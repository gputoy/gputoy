use wasm_bindgen::prelude::wasm_bindgen;

pub struct ClientLogger;
const STATIC_LOGGER: ClientLogger = ClientLogger;

#[wasm_bindgen(module = "/log_ext.js")]
extern "C" {
    fn __trace_ext(log: String);
    fn __debug_ext(log: String);
    fn __info_ext(log: String);
    fn __warn_ext(log: String);
    fn __error_ext(log: String);
}

impl log::Log for ClientLogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        let log = record.args().to_string();

        match record.level() {
            log::Level::Error => __error_ext(log),
            log::Level::Warn => __warn_ext(log),
            log::Level::Info => __info_ext(log),
            log::Level::Debug => __debug_ext(log),
            log::Level::Trace => __trace_ext(log),
        }
    }

    fn flush(&self) {}
}

pub fn init() -> Result<(), log::SetLoggerError> {
    log::set_logger(&STATIC_LOGGER)?;
    log::set_max_level(log::LevelFilter::Trace);
    Ok(())
}
