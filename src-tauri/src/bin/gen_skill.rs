use std::fs::{self, File};
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;

use rafield_lib::domain::gamedb::skills::{SkillConfigData, SkillEventMs, SkillType};
use rafield_lib::domain::types::id::SkillID;
use rafield_lib::infra::repo::{self, save_file};
use serde::{Deserialize, Serialize};
use ustr::Ustr;

#[derive(Debug, Serialize, Deserialize)]
pub struct SkillConfigTemp {
    pub sk_id: SkillID,
    pub sk_type: SkillType,
    pub icon: String,
    pub path: PathBuf,
}

fn gen_temp(skill_type: SkillType, name: &str, index: u8, ver: u8) -> SkillConfigTemp {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let root = PathBuf::from(manifest_dir);
    let base_path = root.parent().unwrap().join("assets").join("data").join("skills");

    let (skill_id, json_path, icon_path) = match skill_type {
        SkillType::BasicAttack => {
            let mut id = format!("sk_{}_basic_00{}", name, index);
            if ver != 0 {
                let v = format!("_v{}", ver);
                id.push_str(&v);
            }
            let json_name = format!("{}.json", &id);
            let dir = base_path.join("operators").join(name).join(json_name);
            let icon = format!("icons/skills/operators/{}/icon_{}.png", name, &id);

            (id, dir, icon)
        }
        SkillType::BattleSkill => {
            let mut id = format!("sk_{}_battle", name);
            if ver != 0 {
                let v = format!("_v{}", ver);
                id.push_str(&v);
            }
            let json_name = format!("{}.json", &id);
            let dir = base_path.join("operators").join(name).join(json_name);
            let icon = format!("icons/skills/operators/{}/icon_{}.png", name, &id);

            (id, dir, icon)
        }
        SkillType::ComboSkill => {
            let mut id = format!("sk_{}_combo", name);
            if ver != 0 {
                let v = format!("_v{}", ver);
                id.push_str(&v);
            }
            let json_name = format!("{}.json", &id);
            let dir = base_path.join("operators").join(name).join(json_name);
            let icon = format!("icons/skills/operators/{}/icon_{}.png", name, &id);

            (id, dir, icon)
        }
        SkillType::Ultimate => {
            let mut id = format!("sk_{}_ultimate", name);
            if ver != 0 {
                let v = format!("_v{}", ver);
                id.push_str(&v);
            }
            let json_name = format!("{}.json", &id);
            let dir = base_path.join("operators").join(name).join(json_name);
            let icon = format!("icons/skills/operators/{}/icon_{}.png", name, &id);

            (id, dir, icon)
        }
        SkillType::GearSetSkill => {
            let mut id = format!("sk_gear_set_{}", name);
            if ver != 0 {
                let v = format!("_v{}", ver);
                id.push_str(&v);
            }
            let json_name = format!("{}.json", &id);
            let dir = base_path.join("gear_sets").join(name).join(json_name);
            let icon = format!("icons/skills/gear_sets/{}/icon_{}.png", name, &id);

            (id, dir, icon)
        }
        SkillType::WeaponSkill => {
            let mut id = format!("sk_weapon_{}", name);
            if ver != 0 {
                let v = format!("_v{}", ver);
                id.push_str(&v);
            }
            let json_name = format!("{}.json", &id);
            let dir = base_path.join("weapons").join(name).join(json_name);
            let icon = format!("icons/skills/weapons/{}/icon_{}.png", name, &id);

            (id, dir, icon)
        }
        SkillType::ChangeControlled => {
            let mut id = format!("sk_common_{}", name);
            if ver != 0 {
                let v = format!("_v{}", ver);
                id.push_str(&v);
            }
            let json_name = format!("{}.json", &id);
            let dir = base_path.join("commons").join(name).join(json_name);
            let icon = format!("icons/skills/commons/{}/icon_{}.png", name, &id);

            (id, dir, icon)
        }
        SkillType::Dodge => {
            let mut id = format!("sk_common_{}", name);
            if ver != 0 {
                let v = format!("_v{}", ver);
                id.push_str(&v);
            }
            let json_name = format!("{}", &id);
            let dir = base_path.join("commons").join(name).join(json_name);
            let icon = format!("icons/skills/commons/{}/icon_{}.png", name, &id);

            (id, dir, icon)
        }
    };

    SkillConfigTemp {
        sk_id: SkillID(Ustr::from(&skill_id)),
        sk_type: skill_type,
        icon: icon_path,
        path: json_path,
    }
}

fn save_skill(config: SkillConfigData, path: &PathBuf) {
    let file = File::create(&path).expect("创建/覆盖文件失败");
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, &config).expect("写入json失败");
}

impl SkillConfigTemp {
}

fn main() {
    // 创建数据
    let config_temp = gen_temp(SkillType::BasicAttack, "surtr", 1, 0);
    

    // 加入数据

    println!("{:#?}", config_temp);

    // 写入json
    match save_file(&config_temp, &config_temp.path) {
        Ok(_) => println!("保存成功{}", &config_temp.path.display()),
        Err(e) => println!("错误: {}", e),
    }
}
