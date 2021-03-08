use std::collections::HashMap;

use html_parser::{Dom, Element, Node, Result};

trait NodeExtension {
    fn maybe_element(&self, element_name: &str) -> Option<&Element>;
    fn maybe_text(&self) -> Option<&str>;
    fn inner_text(&self) -> String;
}

impl NodeExtension for Node {
    fn maybe_element(&self, element_name: &str) -> Option<&Element> {
        if let Node::Element(el) = self {
            if el.name.to_uppercase() == element_name.to_uppercase() {
                Some(el)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn maybe_text(&self) -> Option<&str> {
        if let Node::Text(text) = self {
            Some(text.trim())
        } else {
            None
        }
    }

    fn inner_text(&self) -> String {
        self.into_iter().filter_map(|x| x.maybe_text()).collect::<Vec<_>>().join(" ")
    }
}

pub fn parse_table(table: &str) -> Result<Vec<HashMap<String, String>>> {
    let dom = Dom::parse(table)?;

    let headers = dom.children[0]
        .into_iter()
        .find_map(|x| {
            let tr = x.maybe_element("tr")?;
            let headers = tr
                .children
                .iter()
                .filter_map(|node| node.maybe_element("th").map(|_| node.inner_text()))
                .collect::<Vec<_>>();

            (!headers.is_empty()).then(|| headers)
        })
        .unwrap();

    Ok(dom.children[0]
        .into_iter()
        .filter_map(|node| {
            let el = node.maybe_element("tr")?;

            let items = el
                .children
                .iter()
                .enumerate()
                .filter_map(|(index, node)| node.maybe_element("td").map(|_| (headers[index].to_owned(), node.inner_text())))
                .collect::<HashMap<_, _>>();

            (!items.is_empty()).then(|| items)
        })
        .collect::<Vec<_>>())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    #[test]
    fn parse_table_test() -> std::result::Result<(), Box<dyn Error>> {
        let table = r#"
        <table>
            <tr>
                <th>col1</th>
                <th>col2</th>
                <th>col3</th>
            </tr>
            <tr>
                <td>1</td>
                <td>2</td>
                <td>3</td>
            </tr>
            <tr>
                <td>11</td>
                <td>22</td>
                <td>33</td>
            </tr>
            <tr>
                <td>111</td>
                <td>222</td>
                <td>333</td>
            </tr>
            <tr>
                <td>1111</td>
                <td>2222</td>
                <td>3333</td>
            </tr>
        </table>
        "#;

        let data = parse_table(table)?;

        assert_eq!(data[0]["col1"], "1");
        assert_eq!(data[0]["col2"], "2");
        assert_eq!(data[0]["col3"], "3");

        assert_eq!(data[1]["col1"], "11");
        assert_eq!(data[1]["col2"], "22");
        assert_eq!(data[1]["col3"], "33");

        assert_eq!(data[2]["col1"], "111");
        assert_eq!(data[2]["col2"], "222");
        assert_eq!(data[2]["col3"], "333");

        assert_eq!(data[3]["col1"], "1111");
        assert_eq!(data[3]["col2"], "2222");
        assert_eq!(data[3]["col3"], "3333");

        Ok(())
    }
}
