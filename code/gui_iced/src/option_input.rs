use std::fmt::Debug;

use iced::{Element, Length};

use secalc_core::grid::GridCalculator;

use crate::data_bind::{DataBind, DataBindMessage};

macro_rules! create_input_options {
  ($label_length:expr; $input_length:expr; $($field:ident, $type:ty, $message:ident, $label:expr, $format:expr, $unit:expr);*) => {
    pub struct InputOptions {
      $($field: DataBind<$type>,)*
    }

    impl InputOptions {
      pub fn new(calc: &GridCalculator) -> Self {
        Self {
          $($field: DataBind::new($label, $label_length, calc.$field, format!($format, calc.$field), $input_length, $unit),)*
        }
      }
    }

    #[derive(Clone, Debug)]
    pub enum InputOptionMessage {
      $($message(DataBindMessage),)*
    }

    impl InputOptions {
      pub fn update(&mut self, message: InputOptionMessage, calc: &mut GridCalculator) {
        match message {
          $(InputOptionMessage::$message(m) => self.$field.update(m, &mut calc.$field),)*
        }
      }

      pub fn view(&mut self) -> impl IntoIterator<Item=Element<InputOptionMessage>> {
        vec![
          $(self.$field.view().map(move |s| InputOptionMessage::$message(s)),)*
        ]
      }
    }
  }
}

create_input_options!(Length::Units(250); Length::Units(100);
  gravity_multiplier, f64, GravityMultiplier, "Gravity Multiplier", "{:.1}", "*";
  container_multiplier, f64, ContainerMultiplier, "Container Multiplier", "{:.1}", "*";
  planetary_influence, f64, PlanetaryInfluence, "Planetary Influence", "{:.1}", "*";
  additional_mass, f64, AdditionalMass, "Additional Mass", "{}", "kg";
  ice_only_fill, f64, IceOnlyFill, "Ice-only-fill", "{:.1}", "%";
  ore_only_fill, f64, OreOnlyFill, "Ore-only-fill", "{:.1}", "%";
  any_fill_with_ice, f64, AnyFillWithIce, "Any-fill with Ice", "{:.1}", "%";
  any_fill_with_ore, f64, AnyFillWithOre, "Any-fill with Ore", "{:.1}", "%";
  any_fill_with_steel_plates, f64, AnyFillWithSteelPlates, "Any-fill with Steel Plates", "{:.1}", "%"
);
