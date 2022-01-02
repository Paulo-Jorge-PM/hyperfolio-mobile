use log::Level;
use android_logger::Config;

pub fn logCatPrinter() {
    android_logger::init_once(
        Config::default().with_min_level(Level::Trace));
}

/*
Expanded configuration:

use log::Level;
use android_logger::{Config,FilterBuilder};

pub fn logCatPrinter() {
    android_logger::init_once(
        Config::default()
            .with_min_level(Level::Trace) // limit log level
            .with_tag("mytag") // logs will show under mytag tag
            .with_filter( // configure messages for specific crate
                FilterBuilder::new()
                    .parse("debug,hello::crate=error")
                    .build())
    );

    error!("this is printed by default");
}*/
