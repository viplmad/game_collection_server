use super::ModelInfo;

pub struct GameTag();

impl ModelInfo for GameTag {
    const MODEL_NAME: &'static str = "Relation of Game and Tag";
    const ID_FIELDS: &'static [&'static str] = &["game id", "tag id"];
    const UNIQUE_FIELDS: &'static [&'static str] = GameTag::ID_FIELDS;
}
