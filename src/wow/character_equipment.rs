#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterEquipment {
    pub equipped_items: Vec<EquippedItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EquippedItem {
    pub item: Item,
    pub slot: Slot,
    pub quantity: usize,
    pub context: usize,
    pub bonus_list: Vec<i32>,
    pub quality: Quality,
    pub name: String,
    pub item_class: ItemClass,
    pub item_subclass: ItemClass,
    pub inventory_type: InventoryType,
    pub binding: Binding,
    pub armor: Armor,
    pub stats: Vec<ItemStatistic>,
    pub level: Level,
    pub transmog: Transmogrification,
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
    id: u64,
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
pub struct Armor {
    value: i32,
    display: Display,
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
