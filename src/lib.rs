// The wasm-pack uses wasm-bindgen to build and generate JavaScript binding file.
// Import the wasm-bindgen crate.
use js_sys::Float64Array;
use js_sys::Object;
use js_sys::Reflect;
use treemap::{MapItem, Mappable, Rect, TreemapLayout};
use wasm_bindgen::prelude::*;

// Our Add function
// wasm-pack requires "exported" functions
// to include #[wasm_bindgen]
#[wasm_bindgen]
pub fn create_tree_map(items: Float64Array, bounds: Float64Array) -> Object {
	let layout = TreemapLayout::new();
	let parsed_bounds = parse_bounds(bounds);
	let mut parsed_items: Vec<Box<dyn Mappable>> = parse_items(items);
	let result_layout_items = Object::new();

	layout.layout_items(&mut parsed_items, parsed_bounds);

	for (i, item) in parsed_items.iter().enumerate() {
		let item_bounds = item.bounds();
		let item_bounds_cols: Vec<String> = vec!["x".into(), "y".into(), "w".into(), "h".into()];
		let item = Object::new();

		for item_bounds_col in item_bounds_cols {
			let item_bounds_value = get_field(&item_bounds, &item_bounds_col);
			Reflect::set(&item, &item_bounds_col.into(), &item_bounds_value.into())
				.map_err(|err| println!("{:?}", err))
				.ok();
		}

		Reflect::set(&result_layout_items, &i.into(), &item.into())
			.map_err(|err| println!("{:?}", err))
			.ok();
	}

	return result_layout_items;
}

fn parse_items(items: Float64Array) -> Vec<Box<dyn Mappable>> {
	let mut parsed_items: Vec<Box<dyn Mappable>> = Vec::new();

	for item in items.to_vec() {
		let parsed_item = Box::new(MapItem::with_size(item));
		parsed_items.push(parsed_item);
	}

	return parsed_items;
}

fn parse_bounds(bounds: Float64Array) -> Rect {
	let x = bounds.get_index(0);
	let y = bounds.get_index(1);
	let w = bounds.get_index(2);
	let h = bounds.get_index(3);
	let parsed_bounds: Rect = Rect::from_points(x, y, w, h);

	return parsed_bounds;
}

fn get_field(&my_struct: &Rect, field: &str) -> f64 {
	let x_clone = my_struct.x.clone();
	let y_clone = my_struct.y.clone();
	let w_clone = my_struct.w.clone();
	let h_clone = my_struct.h.clone();

	match field {
		"x" => x_clone,
		"y" => y_clone,
		"w" => w_clone,
		"h" => h_clone,
		_ => 0.into(),
	}
}
