use std::fs::{self, File};
use std::path::Path;
use anyhow::{Result,Context};
use serde_json;
use serde::Deserialize;
use std::io::prelude::*;
fn main() {
    println!("Hello, world!");
    // let cluster = get_cluster_info().unwrap();
    let path = Path::new("/tmp/foo/bar.txt");
    let cluster = parse(path).unwrap();
    println!("cluster: {:?}", cluster)
}

#[allow(dead_code)]
fn get_cluster_info() -> Result<ClusterMap> {
    let config = fs::read_to_string("./src/cluster.json")?;
    let map: ClusterMap = serde_json::from_str(&config)?;
    Ok(map)
}

#[derive(Debug,Deserialize)]
#[allow(dead_code)]
pub struct ClusterMap {
    name: String,
    nodes: Vec<String>,
}

fn parse_impl(mut file: File) -> Result<ClusterMap> {
    let mut contents = String::new();
    file.read_to_string(&mut contents).context("read file contents to string")?;
    let map: ClusterMap = serde_json::from_str(&contents).context("deserialize cluster map to string")?;
    Ok(map)
}

pub fn parse(path: impl AsRef<Path>) -> Result<ClusterMap> {
    let path_str = path.as_ref().to_str().unwrap();
    let file = File::open(&path).context(format!("read file {path_str} failed."))?;
    parse_impl(file)
}