use serde_derive::{Deserialize, Serialize};
// use std::time::{Duration,SystemTime};
use failure::Fallible;
use std::path::PathBuf;
use settings::ron::Config;

use std::fs;
use walkdir::{WalkDir};
/// Modus state.
#[derive(Clone, Deserialize, Serialize, PartialEq,Debug)]
pub enum Mode {
    Off,
    Interval(u64),
}






#[derive(Deserialize, Serialize,Debug)]
pub struct Rule {
    pub command:    String,
    pub interval:   u64,
    pub online:     bool,
    pub prioritat:  u8,
    pub name:       String,
}

impl Default for Rule {
    fn default() -> Self {
        Rule::new(1)
    }
}

impl Rule {
    pub fn new(id:u64) -> Rule {
        Self {
            id:id,
            command: String,
            interval: u64,
            online: false,
            prioritat: 0,
            name: format!("{}-rule",id),
        }
    }
    // pub fn file_name(&self) -> Rule {

    // }
}



// /// Get streams work directory
// pub fn get_directory() -> Fallible<PathBuf> {
//     let path = crate::local::rootdir()?;
//     let path = path.join("rule");
//     if !path.exists() {
//         fs::create_dir_all(&path)?;
//     }
//     Ok(path)
// }

// pub async fn read_all() ->Fallible<Vec<Rule>> {
//     let path = get_directory()?;
//     let mut rules = Vec::new();
//     for entry in WalkDir::new(path).min_depth(1) {
//         let entry = entry?;
//         let metadate = entry.metadata()?;
//         if metadate.is_file(){
//             rules.push(Rule::load_no_fallback(entry.into_path())?);
//         }
//     }
//     Ok(rules)
// }

// pub async fn save(rule: Rule) -> Fallible<()> {
//     let path = get_directory()?.join(format!("/{}-rule.ron",rule.id));
//     rule.write(path.join("rule.ron"))?;
//     Ok(())
// }

// pub async fn read(id:u64) -> Fallible<Rule> {
//     let rule = Rule::load_no_fallback(get_directory()?.join(format!("/{}-rule.ron",id)))?;
//     Ok(rule)
// }

// pub async fn set_stream() -> Result<(),WqaError> {
// }

#[cfg(test)]
mod test {
    // use super::*;
    // use std::fs::File;
    // use std::path::Path;

    #[test]
    fn test_find_all() {

        // File::create(&temp_file()).unwrap();
        // let exercise = Exercise {
            // path: PathBuf::from("example.rs"),
            // mode: Mode::Test,
        // };
        // exercise.clean();
        // assert!(!Path::new(&temp_file()).exists());
    }
}

//
