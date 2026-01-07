use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;

use rafield_lib::domain::gamedb::weapons::WeaponAffixCommon;
use rafield_lib::domain::types::data::AffixTier;
use serde::{Deserialize, Serialize};

type AffixHashMap = HashMap<(WeaponAffixCommon, AffixTier), [f64; 9]>;
#[derive(Debug, Serialize, Deserialize)]
pub struct SaveAffixEntry {
    name: WeaponAffixCommon,
    size: AffixTier,
    value: [f64; 9],
}

fn save_hash_map(db: AffixHashMap, path: &PathBuf) {
    let save_list: Vec<SaveAffixEntry> = db
        .iter()
        .map(|((n, s), v)| SaveAffixEntry {
            name: n.clone(),
            size: s.clone(),
            value: v.clone(),
        })
        .collect();

    let file = File::create(&path).expect("创建/覆盖文件失败");
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, &save_list).expect("写入json失败");
}

fn load_hash_map(path: &PathBuf) -> AffixHashMap {
    let file = File::open(&path).expect("打开文件失败");
    let reader = BufReader::new(file);
    let vec_list:Vec<SaveAffixEntry> = serde_json::from_reader(reader).expect("读取入db失败");
    vec_list.iter()
        .map(|s| ((s.name, s.size), s.value.clone()))
        .collect()
}

fn create_new_affix_hashmap(db: &mut AffixHashMap) {
    let affix_common = WeaponAffixCommon::AtkBoost;
    let small = [
        0.030, 0.054, 0.078, 0.102, 0.126, 0.150, 0.174, 0.198, 0.234,
    ];
    let medium = [
        0.040, 0.072, 0.104, 0.136, 0.168, 0.200, 0.232, 0.264, 0.312,
    ];
    let large = [
        0.050, 0.090, 0.130, 0.170, 0.210, 0.250, 0.290, 0.330, 0.390,
    ];
    db.insert((affix_common, AffixTier::Small), small);
    db.insert((affix_common, AffixTier::Medium), medium);
    db.insert((affix_common, AffixTier::Large), large);
}

fn main() {
    // 初始化
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let root = PathBuf::from(manifest_dir);
    let path: PathBuf = root
        .parent()
        .unwrap()
        .join("assets")
        .join("affix")
        .join("affix_common.json");

    // 调试：打印一下绝对路径，确认位置对不对
    if let Ok(abs_path) = path
        .canonicalize()
        .or_else(|_| std::env::current_dir().map(|p| p.join(&path)))
    {
        println!("正在操作路径: {:?}", abs_path);
    }

    if let Some(parent) = path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).expect("创建文件夹affix失败");
        }
    }
    let mut db: HashMap<(WeaponAffixCommon, AffixTier), [f64; 9]> = if path.exists() {
        load_hash_map(&path)
    } else {
        HashMap::new()
    };

    // 加入词条
    create_new_affix_hashmap(&mut db);
    println!("{:#?}", db);

    // 写入json
    save_hash_map(db, &path);
    println!("保存 [affix_common.json] 成功");
}
