use super::*;
// use std::thread::sleep;
use std::time::{Duration,SystemTime};
use async_std::prelude::*;
use async_std::io;
use async_std::fs;
use async_std::task;

// use linux_embedded_hal::Mio;

use std::fmt;

pub struct Mio {
    path: PathBuf,
    pin: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    In,
    Out,
    High,
    Low,
}

impl From<u8> for Direction {
    fn from(value: u8) -> Self {
        match value {
            0 => Direction::In,
            1 => Direction::Out,
            2 => Direction::High,
            3 => Direction::Low,
            _ => Direction::In,
        }
    }
}

impl From<String> for Direction {
    fn from(value: String) -> Self {
        match value.trim() {
            "in"   =>  Direction::In,
            "out"  =>  Direction::Out,
            "high" =>  Direction::High,
            "low"  =>  Direction::Low,
            _      =>  Direction::In
        }
    }
}
impl From<&str> for Direction {
    fn from(value: &str) -> Self {
        match value {
            "in"   =>  Direction::In,
            "out"  =>  Direction::Out,
            "high" =>  Direction::High,
            "low"  =>  Direction::Low,
             _     =>  Direction::In
        }
    }
}
impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Direction::In =>  return write!(f,"in"),
            Direction::Out => return write!(f,"out"),
            Direction::High =>return write!(f,"high"),
            Direction::Low => return write!(f,"low"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Edge {
    NoInterrupt,
    RisingEdge,
    FallingEdge,
    BothEdges,
}

impl From<u8> for Edge {
    fn from(value: u8) -> Self {
        match value {
            0 => Edge::NoInterrupt,
            1 => Edge::RisingEdge,
            2 => Edge::FallingEdge,
            3 => Edge::BothEdges,
            _ => Edge::NoInterrupt,
        }
    }
}

impl From<String> for Edge {
    fn from(value: String) -> Self {
        match value.trim() {
            "none"    =>  Edge::NoInterrupt,
            "rising"  =>  Edge::RisingEdge,
            "falling" =>  Edge::FallingEdge,
            "both"    =>  Edge::BothEdges,
            _         =>  Edge::NoInterrupt
        }
    }
}
impl From<&str> for Edge {
    fn from(value: &str) -> Self {
        match value {
            "none"   =>   Edge::NoInterrupt,
            "rising"  =>  Edge::RisingEdge,
            "falling" =>     Edge::FallingEdge,
            "both"  =>     Edge::BothEdges,
             _     =>     Edge::NoInterrupt
        }
    }
}
impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Edge::NoInterrupt =>  return write!(f,"none"),
            Edge::RisingEdge  =>  return write!(f,"rising"),
            Edge::FallingEdge =>  return write!(f,"falling"),
            Edge::BothEdges   =>  return write!(f,"both"),
        }
    }
}

use super::gpiodir;

impl Mio {
    pub fn value_path(&self,) -> PathBuf {
        self.path.join("value")
    }
    pub fn direction_path(&self) -> PathBuf {
        self.path.join("direction")
    }
    pub fn edge_path(&self) -> PathBuf {
        self.path.join("edge")
    }
}

impl From<PathBuf> for Mio {
    fn from(path:PathBuf) -> Mio {
        let id = path.file_name()
            .and_then(|filename| filename.to_str())
            .and_then(|filename_str| filename_str.trim_start_matches("gpio").parse::<u64>().ok()).unwrap();
        Mio{
            path:path,
            pin:id,
        }
    }

}
pub async fn set_high(pin:&Mio) -> io::Result<()> {
    let mut file = fs::OpenOptions::new()
        .write(true)
        .open(pin.value_path())
        .await?;
    file.write_all(b"1").await?;
    file.sync_all().await?;
    Ok(())
}
pub async fn set_low(pin:&Mio) -> io::Result<()> {
    let mut file = fs::OpenOptions::new()
        .write(true)
        .open(pin.value_path())
        .await?;
    file.write_all(b"0").await?;
    file.sync_all().await?;
    Ok(())
}
pub async fn set_value(pin:&Mio,value:bool) -> io::Result<()> {
    let mut file = fs::OpenOptions::new()
        .write(true)
        .open(pin.value_path())
        .await?;
    file.write_all(match value {
        true => b"1",
        false => b"0"}).await?;
    file.sync_all().await?;
    Ok(())
}
/// Set this GPIO as either an input or an output
/// The basic values allowed here are Direction::In and Direction::Out which set the Mio as either an input or output respectively. In addition to those, two additional settings of Direction::High and Direction::Low. These both set the Mio as an output but do so with an initial value of high or low respectively. This allows for glitch-free operation.
/// Note that this entry may not exist if the kernel does not support changing the direction of a pin in userspace. If this is the case, you will get an error.
pub async fn get_value(pin:&Mio) -> io::Result<bool> {
    let path = pin.value_path();
    match fs::read_to_string(&path).await?.as_str() {
        "1" => Ok(true),
        _ => Ok(false),
    }
}

/// check pin is exported
pub async fn is_exported(pin:&Mio)-> io::Result<bool> {
    let path = pin.path.clone();
    Ok(fs::metadata(&path).await?.is_dir())
}

pub async fn unexport(pin:&Mio) -> io::Result<()> {
    let id = pin.pin;
    let mut unexport = fs::OpenOptions::new()
        .write(true)
        .open(gpiodir().join("unexport"))
        .await?;
    let num = format!("{}",id);
    unexport.write_all(num.as_bytes()).await?;
    unexport.sync_all().await?;
    Ok(())

}

/// export pin
pub async fn export(pin_num: u64) -> io::Result<Mio> {
    let pin_name = format!("gpio{}", pin_num);
    let path = gpiodir().join(&pin_name);
    let value = format!("{}", pin_num);
    if fs::metadata(&path).await.is_err() {
        info!("export GPIO:{} {:?}", pin_num,path);
        let mut export = fs::OpenOptions::new()
            .write(true)
            .open(gpiodir().join("export"))
            .await?;
        export.write_all( value.as_bytes()).await?;
        export.sync_all().await?;
   }
    Ok(Mio {path:path,pin:pin_num})
}

///read pin direction
pub async fn direction(pin: &Mio)-> io::Result<String> {
    let path = pin.direction_path();
    Ok(fs::read_to_string(&path).await?)

}
///read pin edge
pub async fn edge(pin: &Mio)-> io::Result<String> {
    let path = pin.edge_path();
    Ok(fs::read_to_string(&path).await?)
}
/// change direction
pub async fn set_direction(pin:&Mio,dir:Direction) -> io::Result<()> {
    let path = pin.direction_path();
    let dir = format!("{}",dir);
    info!("export GPIO:{} {:?} direction {}", pin.pin,path,dir);
    let mut file = fs::OpenOptions::new() .write(true) .open(&path) .await?;
    file.write_all(dir.as_bytes()).await?;
    file.sync_all().await?;
    Ok(())
}

/// change edge
pub async fn set_edge(pin:&Mio,edge:Edge) -> io::Result<()> {
    let path = pin.edge_path();
    let edge = format!("{}",edge);
    info!("GPIO:{} {:?} edge {}", pin.pin,path,edge);
    let mut file = fs::OpenOptions::new() .write(true) .open(&path) .await?;
    file.write_all(edge.as_bytes()).await?;
    file.sync_all().await?;
    Ok(())
}

impl fmt::Display for Mio {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GPIO{}:{:?}", self.pin,self.path)
    }
}
//
