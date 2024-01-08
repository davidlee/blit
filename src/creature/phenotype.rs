use crate::typical::*;

// Phenotype
//
#[derive(Component, Debug, Clone, Default, Eq, PartialEq)]
#[allow(dead_code)]
pub struct Phenotype {
    size: CreatureSize,
    anatomy_template: (),

    natural_weapons: (),
    natural_armour: (),
    natural_inventory: (),

    innate_abilities: (),
    traits: (),
    // metabolism
    // needs
    // thoughts ..
}

#[allow(dead_code)]
impl Phenotype {
    fn default() -> Self {
        Phenotype {
            size: CreatureSize::default(),
            anatomy_template: (),

            natural_weapons: (),
            natural_armour: (),
            natural_inventory: (),
            innate_abilities: (),
            traits: (),
        }
    }
}
