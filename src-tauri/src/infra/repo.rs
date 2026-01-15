use std::fs::{self, File};
use std::io::{BufReader, BufWriter};
use std::path::{Path, PathBuf};

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

pub fn save_file<T>(save_data: &T, path: &Path) -> Result<(), String> where T: Serialize {
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("创建目录失败 {:?}: {}", parent, e))?;
        }
    }

    let file = File::create(&path)
        .map_err(|e| format!("创建文件失败 {:?}: {}", path, e))?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, &save_data)
        .map_err(|e| format!("序列化JSON失败 {:?}: {}", path, e))?;

    Ok(())
}

pub fn load_file<T>(path: &Path)  -> Result<T, String> where T: DeserializeOwned{
    let file = File::open(&path)
                .map_err(|e| format!("打开文件失败 {:?}: {}", path, e))?;
    let reader = BufReader::new(file);
    let data: T = serde_json::from_reader(reader)
                .map_err(|e| format!("反序列化JSON失败 {:?}: {}", path, e))?;
    
    Ok(data)
} 
