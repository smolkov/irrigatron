use async_log::{instrument, span};
use log::info;

fn setup_logger() {
    let logger = femme::pretty::Logger::new();
    async_log::Logger::wrap(logger, || /* get the task id here */ 0)
        .start(log::LevelFilter::Trace)
        .unwrap();
}
// use log::{LevelFilter,SetLoggerError,info};
// use log::info;
// use std::{
    // io::{self},
    // fs::{create_dir_all},
    // path::{PathBuf},
// };

// use std::{
    // io::{self},
    // fs::{create_dir_all},
    // path::{PathBuf},
// };


// pub fn setup_logger() {

    // let logger  env_logger::Builder::new()
    //     .filter(None, log::LevelFilter::Trace)
    //     .build();

    // async_log::Logger::wrap(logger, || /* get the task id here */ 0)
    //     .start(log::LevelFilter::Trace)
    //     .unwrap();
    // span!("new level, depth={}", 1, {
    //     let x = "beep";
    //     info!("look at this value, x={}", x);

    //     span!("new level, depth={}", 2, {
    //         inner("boop");
    //     })
    // });
// }


