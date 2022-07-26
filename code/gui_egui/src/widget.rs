use std::fmt::Display;
use std::hash::Hash;
use std::ops::RangeInclusive;

use egui::{Button, CollapsingHeader, CollapsingResponse, Context, DragValue, Grid, InnerResponse, Response, Ui, WidgetText, Window};
use egui::emath::Numeric;

// Context extensions

pub trait CtxRefWidgetsExt {
  fn window(&self, title: impl Into<WidgetText>, add_contents: impl FnOnce(&mut Ui)) -> Option<InnerResponse<Option<()>>>;
}

impl CtxRefWidgetsExt for &Context {
  #[inline]
  fn window(&self, title: impl Into<WidgetText>, add_contents: impl FnOnce(&mut Ui)) -> Option<InnerResponse<Option<()>>> {
    Window::new(title).show(self, add_contents)
  }
}


// Ui extensions

pub trait UiWidgetsExt where {
  fn collapsing_open<R>(&mut self, heading: impl Into<WidgetText>, add_contents: impl FnOnce(&mut Ui) -> R) -> CollapsingResponse<R>;
  fn grid<R>(&mut self, id_source: impl Hash, add_contents: impl FnOnce(&mut Ui) -> R) -> InnerResponse<R>;
  fn collapsing_with_grid<R>(&mut self, heading: impl Into<WidgetText>, grid_id: impl Hash, add_contents: impl FnOnce(&mut Ui) -> R) -> CollapsingResponse<InnerResponse<R>>;
  fn collapsing_open_with_grid<R>(&mut self, heading: impl Into<WidgetText>, grid_id: impl Hash, add_contents: impl FnOnce(&mut Ui) -> R) -> CollapsingResponse<InnerResponse<R>>;

  fn reset_button<T: Default + PartialEq>(&mut self, value: &mut T);
  fn reset_button_with<T: PartialEq>(&mut self, value: &mut T, reset_value: T);
  fn reset_button_double_click<T: Default + PartialEq>(&mut self, value: &mut T);
  fn reset_button_double_click_with<T: PartialEq>(&mut self, value: &mut T, reset_value: T);
  fn reset_button_response(&mut self, can_reset: bool) -> Response;

  fn drag<N: Numeric>(&mut self, prefix: impl ToString, value: &mut N, speed: impl Into<f64>) -> Response;
  fn drag_with_reset<N: Numeric>(&mut self, prefix: impl ToString, value: &mut N, speed: impl Into<f64>, reset_value: N) -> Response;
  fn drag_range<N: Numeric>(&mut self, prefix: impl ToString, value: &mut N, speed: impl Into<f64>, clamp_range: RangeInclusive<N>) -> Response;
  fn drag_range_with_reset<N: Numeric>(&mut self, prefix: impl ToString, value: &mut N, speed: impl Into<f64>, clamp_range: RangeInclusive<N>, reset_value: N) -> Response;
  fn drag_unlabelled<N: Numeric>(&mut self, value: &mut N, speed: impl Into<f64>) -> Response;
  fn drag_unlabelled_with_reset<N: Numeric>(&mut self, value: &mut N, speed: impl Into<f64>, reset_value: N) -> Response;
  fn drag_unlabelled_range<N: Numeric>(&mut self, value: &mut N, speed: impl Into<f64>, clamp_range: RangeInclusive<N>) -> Response;
  fn drag_unlabelled_range_with_reset<N: Numeric>(&mut self, value: &mut N, speed: impl Into<f64>, clamp_range: RangeInclusive<N>, reset_value: N) -> Response;

  fn show_f32_2(&mut self, float: f32);
  fn show_f32_lp_5_2(&mut self, float: f32);
  fn show_f32_lp_7_2(&mut self, float: f32);
  fn show_prefixed_f32_lp_7_2(&mut self, prefix: impl Display, float: f32);
}


impl UiWidgetsExt for Ui {
  #[inline]
  fn collapsing_open<R>(
    &mut self,
    heading: impl Into<WidgetText>,
    add_contents: impl FnOnce(&mut Ui) -> R,
  ) -> CollapsingResponse<R> {
    CollapsingHeader::new(heading).default_open(true).show(self, add_contents)
  }
  #[inline]
  fn grid<R>(&mut self, id_source: impl Hash, add_contents: impl FnOnce(&mut Ui) -> R) -> InnerResponse<R> {
    Grid::new(id_source).striped(true).show(self, add_contents)
  }
  #[inline]
  fn collapsing_with_grid<R>(&mut self, heading: impl Into<WidgetText>, grid_id: impl Hash, add_contents: impl FnOnce(&mut Ui) -> R) -> CollapsingResponse<InnerResponse<R>> {
    self.collapsing(heading, |ui| { ui.grid(grid_id, add_contents) })
  }
  #[inline]
  fn collapsing_open_with_grid<R>(&mut self, heading: impl Into<WidgetText>, grid_id: impl Hash, add_contents: impl FnOnce(&mut Ui) -> R) -> CollapsingResponse<InnerResponse<R>> {
    self.collapsing_open(heading, |ui| { ui.grid(grid_id, add_contents) })
  }


  #[inline]
  fn reset_button<T: Default + PartialEq>(&mut self, value: &mut T) {
    self.reset_button_with(value, T::default());
  }
  #[inline]
  fn reset_button_with<T: PartialEq>(&mut self, value: &mut T, reset_value: T) {
    if self.reset_button_response(*value != reset_value).clicked() {
      *value = reset_value;
    }
  }
  #[inline]
  fn reset_button_double_click<T: Default + PartialEq>(&mut self, value: &mut T) {
    self.reset_button_double_click_with(value, T::default());
  }
  #[inline]
  fn reset_button_double_click_with<T: PartialEq>(&mut self, value: &mut T, reset_value: T) {
    if self.reset_button_response(*value != reset_value).double_clicked() {
      *value = reset_value;
    }
  }
  #[inline]
  fn reset_button_response(&mut self, can_reset: bool) -> Response {
    self.add_enabled(can_reset, Button::new("↺"))
  }


  #[inline]
  fn drag<N: Numeric>(&mut self, prefix: impl ToString, value: &mut N, speed: impl Into<f64>) -> Response {
    self.add(DragValue::new(value).prefix(prefix).speed(speed))
  }
  #[inline]
  fn drag_with_reset<N: Numeric>(&mut self, prefix: impl ToString, value: &mut N, speed: impl Into<f64>, reset_value: N) -> Response {
    self.horizontal(|ui| {
      let response = ui.drag(prefix, value, speed);
      ui.reset_button_with(value, reset_value);
      response
    }).response
  }
  #[inline]
  fn drag_range<N: Numeric>(&mut self, prefix: impl ToString, value: &mut N, speed: impl Into<f64>, clamp_range: RangeInclusive<N>) -> Response {
    self.add(DragValue::new(value).prefix(prefix).speed(speed).clamp_range(clamp_range))
  }
  #[inline]
  fn drag_range_with_reset<N: Numeric>(&mut self, prefix: impl ToString, value: &mut N, speed: impl Into<f64>, clamp_range: RangeInclusive<N>, reset_value: N) -> Response {
    self.horizontal(|ui| {
      let response = ui.drag_range(prefix, value, speed, clamp_range);
      ui.reset_button_with(value, reset_value);
      response
    }).response
  }
  #[inline]
  fn drag_unlabelled<N: Numeric>(&mut self, value: &mut N, speed: impl Into<f64>) -> Response {
    self.add(DragValue::new(value).speed(speed))
  }
  #[inline]
  fn drag_unlabelled_with_reset<N: Numeric>(&mut self, value: &mut N, speed: impl Into<f64>, reset_value: N) -> Response {
    self.horizontal(|ui| {
      let response = ui.drag_unlabelled(value, speed);
      ui.reset_button_with(value, reset_value);
      response
    }).response
  }
  #[inline]
  fn drag_unlabelled_range<N: Numeric>(&mut self, value: &mut N, speed: impl Into<f64>, clamp_range: RangeInclusive<N>) -> Response {
    self.add(DragValue::new(value).speed(speed).clamp_range(clamp_range))
  }
  #[inline]
  fn drag_unlabelled_range_with_reset<N: Numeric>(&mut self, value: &mut N, speed: impl Into<f64>, clamp_range: RangeInclusive<N>, reset_value: N) -> Response {
    self.horizontal(|ui| {
      let response = ui.drag_unlabelled_range(value, speed, clamp_range);
      ui.reset_button_with(value, reset_value);
      response
    }).response
  }


  #[inline]
  fn show_f32_2(&mut self, float: f32) {
    self.monospace(format!("{:.2}", float));
  }
  #[inline]
  fn show_f32_lp_5_2(&mut self, float: f32) {
    self.monospace(format!("{:>5.2}", float));
  }
  #[inline]
  fn show_f32_lp_7_2(&mut self, float: f32) {
    self.monospace(format!("{:>7.2}", float));
  }
  #[inline]
  fn show_prefixed_f32_lp_7_2(&mut self, prefix: impl Display, float: f32) {
    self.monospace(format!("{}: {:>7.2}", prefix, float));
  }
}