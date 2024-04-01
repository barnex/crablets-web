use bevy_reflect::{Array, List, Reflect, ReflectMut, ReflectRef, Struct, Tuple, TupleStruct};
use egui::Ui;

// UI for inspecting and modifying data via refection.
pub fn inspect(ui: &mut Ui, name: &str, value: &mut dyn Reflect) {
	egui::ScrollArea::vertical()
		.scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysVisible)
		.show(ui, |ui| {
			inspect_recursive(ui, name, value, 0);
		});
}

fn inspect_recursive(ui: &mut Ui, name: &str, value: &mut dyn Reflect, depth: u32) {
	match value.reflect_mut() {
		ReflectMut::Value(value) => inspect_value(ui, name, value, depth),
		ReflectMut::Struct(value) => inspect_struct(ui, name, value, depth),
		ReflectMut::TupleStruct(value) => inspect_tuple_struct(ui, name, value, depth),
		ReflectMut::Array(value) => inspect_array(ui, name, value, depth),
		ReflectMut::Tuple(value) => inspect_tuple(ui, name, value, depth),
		ReflectMut::List(value) => inspect_list(ui, name, value, depth),
		ReflectMut::Map(_) => inspect_unsupported(ui, name, value, depth),
		ReflectMut::Enum(_) => inspect_unsupported(ui, name, value, depth),
	};
}

// name: <value>
fn inspect_value(ui: &mut Ui, name: &str, value: &mut dyn Reflect, _depth: u32) {
	ui.horizontal(|ui| {
		ui.label(name);
		inspect_field(ui, value);
	});
}

// <value>
fn inspect_field(ui: &mut Ui, value: &mut dyn Reflect) {
	let any = value.as_any_mut();

	if let Some(ptr) = any.downcast_mut::<f32>() {
		return drop(ui.add(egui::DragValue::new(ptr).speed(0.1)));
	}

	if let Some(ptr) = any.downcast_mut::<i32>() {
		return drop(ui.add(egui::DragValue::new(ptr)));
	}

	if let Some(ptr) = any.downcast_mut::<u32>() {
		return drop(ui.add(egui::DragValue::new(ptr)));
	}

	if let Some(ptr) = any.downcast_mut::<bool>() {
		return drop(ui.add(egui::Checkbox::new(ptr, "")));
	}

	if let Some(ptr) = any.downcast_mut::<String>() {
		return drop(ui.text_edit_singleline(ptr));
	}

	if let Some(ptr) = any.downcast_mut::<u8>() {
		return drop(ui.add(egui::DragValue::new(ptr)));
	}

	if let Some(ptr) = any.downcast_mut::<u16>() {
		return drop(ui.add(egui::DragValue::new(ptr)));
	}

	if let Some(ptr) = any.downcast_mut::<i16>() {
		return drop(ui.add(egui::DragValue::new(ptr)));
	}

	if let Some(ptr) = any.downcast_mut::<usize>() {
		return drop(ui.add(egui::DragValue::new(ptr)));
	}

	ui.label(format!("{value:?}"));
}

// name:
//  field: <value>
//  field: <value>
// (omit name, collapsing header at top level)
fn inspect_struct(ui: &mut Ui, name: &str, value: &mut dyn Struct, depth: u32) {
	match depth {
		0 => inspect_struct_inner(ui, value, depth + 1),
		_ => drop(ui.collapsing(name, |ui| inspect_struct_inner(ui, value, depth + 1))),
	}
}

fn inspect_struct_inner(ui: &mut Ui, value: &mut dyn Struct, depth: u32) {
	for i in 0..value.field_len() {
		inspect_recursive(ui, &value.name_at(i).expect("field name").to_owned(), value.field_at_mut(i).expect("field i"), depth + 1);
	}
}

//   name: <value> <value>  (for primitive values)
//
// or
//
//   name:
//      <value>
//      <value>  (for compound values)
fn inspect_array(ui: &mut Ui, name: &str, value: &mut dyn Array, depth: u32) {
	if value.is_empty() {
		return;
	}
	if matches!(value.get(0).expect("field 0").reflect_ref(), ReflectRef::Value(_)) {
		// atomic elements are shown in-line (not collapsing)
		ui.horizontal(|ui| {
			ui.label(name);
			for i in 0..value.len() {
				inspect_field(ui, value.get_mut(i).expect("field i"));
			}
		});
	} else {
		// composite elements are collapsed
		ui.label(name);
		for i in 0..value.len() {
			inspect_recursive(ui, &i.to_string(), value.get_mut(i).expect("field i"), depth + 1);
		}
	}
}

// >name:
//   <value>
//   <value>
fn inspect_list(ui: &mut Ui, name: &str, value: &mut dyn List, depth: u32) {
	let label = format!("{name} [{}]", value.len());
	ui.collapsing(&label, |ui| {
		for i in 0..value.len() {
			inspect_recursive(ui, &i.to_string(), value.get_mut(i).expect("field i"), depth + 1);
		}
	});
}

// name: <value> <value>
fn inspect_tuple(ui: &mut Ui, name: &str, value: &mut dyn Tuple, _depth: u32) {
	ui.horizontal(|ui| {
		ui.label(name);
		for i in 0..value.field_len() {
			inspect_field(ui, value.field_mut(i).expect("field i"));
		}
	});
}

// name: <value> <value>
fn inspect_tuple_struct(ui: &mut Ui, name: &str, value: &mut dyn TupleStruct, depth: u32) {
	if value.field_len() == 1 {
		// special case for newtype
		inspect_recursive(ui, name, value.field_mut(0).expect("field 0"), depth + 1);
	} else {
		ui.horizontal(|ui| {
			ui.label(name);
			for i in 0..value.field_len() {
				inspect_field(ui, value.field_mut(i).expect("field i"));
			}
		});
	}
}

// name: <text representation>
fn inspect_unsupported(ui: &mut Ui, name: &str, value: &mut dyn Reflect, _depth: u32) {
	ui.horizontal(|ui| {
		ui.label(name);
		inspect_field(ui, value);
	});
}
