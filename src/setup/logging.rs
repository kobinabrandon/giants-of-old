pub fn setup_logging() {
    env_logger::Builder::from_default_env()
                        .filter_level(log::LevelFilter::Info)   
                        .init();
}

