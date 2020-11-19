#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CharacterEquipment {
    pub equipped_items: Vec<EquippedItem>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EquippedItem {
    pub item: Item,
    pub slot: Slot,
    pub quantity: usize,
    pub context: Option<usize>,
    pub bonus_list: Option<Vec<i32>>,
    pub quality: Quality,
    pub name: String,
    pub item_class: ItemClass,
    pub item_subclass: ItemClass,
    pub inventory_type: InventoryType,
    pub binding: Binding,
    pub armor: Option<Armor>,
    pub stats: Option<Vec<ItemStatistic>>,
    pub level: Level,
    pub transmog: Option<Transmogrification>,
    pub durability: Option<Durability>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Slot {
    #[serde(alias = "type")]
    type_: String,
    name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InventoryType {
    #[serde(alias = "type")]
    type_: String,
    name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Quality {
    #[serde(alias = "type")]
    type_: String,
    name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Binding {
    #[serde(alias = "type")]
    type_: String,
    name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ItemClass {
    id: u64,
    name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Level {
    value: i32,
    display_string: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Durability {
    value: i32,
    display_string: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Armor {
    value: i32,
    display: Display,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Item {
    id: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Display {
    display_string: String,
    color: Color,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Color {
    r: usize,
    g: usize,
    b: usize,
    a: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StatisticType {
    #[serde(alias = "type")]
    type_: String,
    name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ItemStatistic {
    #[serde(alias = "type")]
    type_: StatisticType,
    value: i32,
    is_negated: Option<bool>,
    display: Display,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transmogrification {
    item: TransmogrificationItem,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransmogrificationItem {
    id: i32,
    name: String,
}
