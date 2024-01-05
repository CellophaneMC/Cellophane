use cellophanemc_world::dimension::DimensionType;

fn main() {
    let j = include_str!("../assets/minecraft/dimension_type/overworld.json");
    let result = serde_json::from_str::<DimensionType>(j).unwrap();
    println!("{:?}", result);
    let value1 = cellophanemc_nbt::aa::to_value(result).unwrap();
    println!("{:?}", value1);
}
