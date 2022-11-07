pub struct ItemDef {
    appid: u32,
    items: Vec<Item>,
}

pub enum ItemType {
    Item,
    Bundle,
    Generator,
    PlayTimeGenerator,
    TagGenerator,
}

impl ItemType {
    fn to_string(&self) -> &str {
        match self {
            ItemType::Item => "item",
            ItemType::Bundle => "bundle",
            ItemType::Generator => "generator",
            ItemType::PlayTimeGenerator => "playtimegenerator",
            ItemType::TagGenerator => "tag_generator",
        }
    }
}

pub struct Item {
    itemdef_id: u32,
    timestamp: String,
    modified: String,
    date_created: String,
    type_: ItemType,
    display_type: String,
    name: String,
    description: String,
    icon_url: String,
    icon_url_large: String,
    name_color: String,
    tradeable: bool,
    marketable: bool,
    commodity: bool,
}
