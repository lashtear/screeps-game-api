use crate::objects::{OwnedStructure, RoomObject, Structure, Store};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// An object representing a [`StructurePowerSpawn`], which can process power to contribute to your GPL as well as renewing power creeps.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructurePowerSpawn)
    #[wasm_bindgen(extends = RoomObject, extends = Structure, extends = OwnedStructure)]
    pub type StructurePowerSpawn;

    /// The [`Store`] of the power spawn, which can contain power and energy.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructurePowerSpawn.store)
    #[wasm_bindgen(method, getter)]
    pub fn store(this: &StructurePowerSpawn) -> Store;

    /// Process power, consuming 1 power and [`POWER_SPAWN_ENERGY_RATIO`] energy and increasing your GPL by one point.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructurePowerSpawn.processPower)
    ///
    /// [`POWER_SPAWN_ENERGY_RATIO`]: crate::constants::numbers::POWER_SPAWN_ENERGY_RATIO
    #[wasm_bindgen(method)]
    pub fn process_power(this: &StructurePowerSpawn) -> i8;
}


// use crate::{constants::ReturnCode, objects::StructurePowerSpawn};

// impl StructurePowerSpawn {
//     pub fn process_power(&self) -> ReturnCode {
//         js_unwrap! {@{self.as_ref()}.processPower()}
//     }
// }
