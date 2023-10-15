use std::fs::File;
use std::io::Read;
use bevy::prelude::Resource;

#[derive(Resource)]
#[derive(Debug, Eq, PartialEq)]
#[derive(serde::Deserialize)]
pub struct Settings {
    pub theme: String,
    pub row_tile_count: i32,
    pub col_tile_count: i32,
    pub row_hint_count: i32,
    pub col_hint_count: i32
}

impl Settings {
    pub fn new() -> Self {
        let mut file = File::open("settings/settings.json").unwrap();

        // Read the file contents into a string
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let settings: Settings = serde_json::from_str(&contents).unwrap();
        return settings;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_settings() {
        let settings = Settings::new();
        assert_eq!(settings.row_tile_count, 8);
        assert_eq!(settings.col_tile_count, 8);
        assert_eq!(settings.theme, "default");
        assert_eq!(settings.row_hint_count, 3);
        assert_eq!(settings.col_hint_count, 3);
    }
}