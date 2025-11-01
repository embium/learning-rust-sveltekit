use byte_unit::Byte;

// Internal
lazy_static! {
    pub static ref VERSION: String = option_env!("CARGO_PKG_VERSION")
        .unwrap_or("Unknown")
        .to_string();
    pub static ref FRONTEND_PATH: String =
        std::env::var("FRONTEND_PATH").unwrap_or("../frontend/build".to_string());
    pub static ref LISTEN_ADDR: String =
        std::env::var("LISTEN_ADDR").unwrap_or("0.0.0.0:8000".to_string());
    pub static ref VERBOSITY: String = std::env::var("VERBOSITY").unwrap_or("warn".to_string());
    pub static ref LIMIT: usize =
      Byte::from_str(std::env::var("SIZE_LIMIT").unwrap_or("1 KiB".to_string()))
        .unwrap()
        .get_bytes() as usize;
}
