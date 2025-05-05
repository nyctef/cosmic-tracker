use serde::Serialize;
use serde_wasm_bindgen;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[derive(Serialize)]
struct TableRow {
    cells: Vec<String>,
}

#[wasm_bindgen]
pub fn get_table_data() -> JsValue {
    let data = vec![
        TableRow {
            cells: vec!["Row 1, Col 1".to_string(), "Row 1, Col 2".to_string()],
        },
        TableRow {
            cells: vec!["Row 2, Col 1".to_string(), "Row 2, Col 2".to_string()],
        },
    ];

    serde_wasm_bindgen::to_value(&data).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
