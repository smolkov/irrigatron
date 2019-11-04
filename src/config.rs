///Irrigaton config.
 use serde::{Deserialize, Serialize};


// use failure::{
    // Fallible,
// };
/// Configuration
 #[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {

}



pub fn read(_path: &str) -> super::Result<Config> {
    Ok(Config{})
}


pub fn write(_config: Config) ->super::Result<()> {

    Ok(())
} 