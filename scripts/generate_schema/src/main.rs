use std::{fs::File, io::Write};

use kaledis::{
    self, schemars, serde_json,
    toml_conf::{KaledisConfig, LoveConfig},
};

pub fn main() {
    let schema = schemars::schema_for!(KaledisConfig);
    let schema2 = schemars::schema_for!(LoveConfig);
    File::create("kaledis.schema.json")
        .unwrap()
        .write_all(serde_json::to_string(&schema).unwrap().as_bytes())
        .unwrap();
    File::create("love.schema.json")
        .unwrap()
        .write_all(serde_json::to_string(&schema2).unwrap().as_bytes())
        .unwrap();
}
