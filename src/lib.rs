use wasm_bindgen::prelude::*;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[wasm_bindgen]
pub fn update_table() -> String {
    let data = vec![
        vec!["Row 1, Col 1", "Row 1, Col 2"],
        vec!["Row 2, Col 1", "Row 2, Col 2"],
    ];

    let mut table_html = String::from("<table>");
    for row in data {
        table_html.push_str("<tr>");
        for cell in row {
            table_html.push_str(&format!("<td>{}</td>", cell));
        }
        table_html.push_str("</tr>");
    }
    table_html.push_str("</table>");

    table_html
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
