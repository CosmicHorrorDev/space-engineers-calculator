use std::path::Path;

use blocks::Blocks;
use components::Components;
use gas_properties::GasProperties;
use localization::Localization;

pub mod blocks;
pub mod components;
pub mod gas_properties;
pub mod localization;
pub mod xml;

pub struct Data {
  pub blocks: Blocks,
  pub components: Components,
  pub gas_properties: GasProperties,
  pub localization: Localization,
}

impl Data {
  pub fn from_se_dir<P: AsRef<Path> + Copy>(se_dir: P) -> Self {
    let blocks = Blocks::from_se_dir(se_dir);
    let components = Components::from_se_dir(se_dir);
    let gas_properties = GasProperties::from_se_dir(se_dir);
    let localization = Localization::from_se_dir(se_dir);
    Self { blocks, components, gas_properties, localization }
  }

  pub fn debug_print(&self) {
    dbg!(&self.components);
    dbg!(&self.gas_properties);

    for thruster in self.blocks.thrusters.values() {
      dbg!(thruster);
      dbg!(thruster.name(&self.localization));
      dbg!(thruster.mass(&self.components));
    }

    for battery in self.blocks.batteries.values() {
      dbg!(battery);
      dbg!(battery.name(&self.localization));
      dbg!(battery.mass(&self.components));
    }

    for hydrogen_engine in self.blocks.hydrogen_engines.values() {
      dbg!(hydrogen_engine);
      dbg!(hydrogen_engine.name(&self.localization));
      dbg!(hydrogen_engine.mass(&self.components));
    }

    for reactor in self.blocks.reactors.values() {
      dbg!(reactor);
      dbg!(reactor.name(&self.localization));
      dbg!(reactor.mass(&self.components));
    }

    for generator in self.blocks.generators.values() {
      dbg!(generator);
      dbg!(generator.name(&self.localization));
      dbg!(generator.mass(&self.components));
    }

    for hydrogen_tank in self.blocks.hydrogen_tanks.values() {
      dbg!(hydrogen_tank);
      dbg!(hydrogen_tank.name(&self.localization));
      dbg!(hydrogen_tank.mass(&self.components));
    }

    for container in self.blocks.containers.values() {
      dbg!(container);
      dbg!(container.name(&self.localization));
      dbg!(container.mass(&self.components));
    }

    for cockpit in self.blocks.cockpits.values() {
      dbg!(cockpit);
      dbg!(cockpit.name(&self.localization));
      dbg!(cockpit.mass(&self.components));
    }
  }
}
