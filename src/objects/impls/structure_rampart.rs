use crate::{
    objects::{OwnedStructure, RoomObject, Structure},
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// An object representing a [`StructureRampart`], which is selectively walkable and protects creeps and structures at the same position.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureRampart)
    #[wasm_bindgen(extends = RoomObject, extends = Structure, extends = OwnedStructure)]
    pub type StructureRampart;

    /// Whether the [`StructureRampart`] is set to be public, allowing hostile creeps to walk on it.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureRampart.isPublic)
    #[wasm_bindgen(method, getter = isPublic)]
    pub fn is_public(this: &StructureRampart) -> bool;

    /// The number of ticks until the rampart will decay, losing [`RAMPART_DECAY_AMOUNT`] hits.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureRampart.ticksToDecay)
    ///
    /// [`RAMPART_DECAY_AMOUNT`]: crate::constants::numbers::RAMPART_DECAY_AMOUNT
    #[wasm_bindgen(method, getter = ticksToDecay)]
    pub fn ticks_to_decay(this: &StructureRampart) -> u32;

    /// Set whether [`StructureRampart`] is public, allowing hostile creeps to walk on it.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureRampart.setPublic)
    #[wasm_bindgen(method, js_name = setPublic)]
    pub fn set_public(this: &StructureRampart, val: bool) -> i8;
}


// use crate::{constants::ReturnCode, objects::StructureRampart};

// simple_accessors! {
//     impl StructureRampart {
//         pub fn is_public() -> bool = isPublic;
//     }
// }

// impl StructureRampart {
//     pub fn set_public(&self, is_public: bool) -> ReturnCode {
//         js_unwrap! { @{self.as_ref()}.setPublic( @{is_public} ) }
//     }
// }
