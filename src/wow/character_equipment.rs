#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterEquipment {
    pub name: String,
    pub equipped_items: Vec<EquippedItem>,
    pub quantity: usize,
    pub context: usize,
    pub bonus_list: Vec<i32>,
    pub stats: Vec<ItemStatistic>,
    pub transmog: Transmogrification,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EquippedItem {
    pub slot: Slot,
    pub inventory_type: InventoryType,
    pub item: Item,
    pub quality: Quality,
    pub item_class: ItemClass,
    pub item_subclass: ItemClass,
    pub level: Level,
    pub durability: Durability,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Slot {
    #[serde(alias = "type")]
    type_: String,
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InventoryType {
    #[serde(alias = "type")]
    type_: String,
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Quality {
    #[serde(alias = "type")]
    type_: String,
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Binding {
    #[serde(alias = "type")]
    type_: String,
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemClass {
    id: usize,
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Level {
    value: i32,
    display_string: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Durability {
    value: i32,
    display_string: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Display {
    display_string: String,
    color: Color,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Color {
    r: usize,
    g: usize,
    b: usize,
    a: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatisticType {
    #[serde(alias = "type")]
    type_: String,
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemStatistic {
    #[serde(alias = "type")]
    type_: StatisticType,
    value: i32,
    is_negated: bool,
    display: Display,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Transmogrification {
    item: TransmogrificationItem,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransmogrificationItem {
    id: i32,
    name: String,
}
