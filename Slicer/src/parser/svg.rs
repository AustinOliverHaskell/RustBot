use quick_xml::Reader;
use quick_xml::events::Event;

pub fn parse_svg_viewbox(element: &quick_xml::events::BytesStart) -> Result<(f32, f32), String> {
    let attribute_list = element.attributes();

    let mut viewbox_height: Option<f32> = None;
    let mut viewbox_width : Option<f32> = None;
    for attribute in attribute_list {
        let att = attribute.unwrap();
        if att.key == b"viewBox" {
            let value: String = String::from_utf8(att.value.to_vec()).unwrap_or(String::from(""));
            let possible_view_box = parse_viewbox(value);

            let view_box: (f32, f32);
            match possible_view_box {
                Ok(val) => view_box = val,
                Err(e) => return Err(e)
            }

            viewbox_height = Some(view_box.1);
            viewbox_width  = Some(view_box.0);
        }
    }

    if viewbox_height.is_none() || viewbox_width.is_none() {
        return Err(String::from(""))
    }

    Ok((viewbox_width.unwrap(), viewbox_height.unwrap()))
}

fn parse_viewbox(s: String) -> Result<(f32, f32), String> {
    let items: Vec<&str> = s.split(' ').collect();
    if items.len() < 4 {
        return Err(String::from("Error: viewbox data is malformed. Expected 4 items, got") + &items.len().to_string());
    }

    let width: Result<f32, _>  = items[2].parse();
    let height: Result<f32, _> = items[3].parse();

    if width.is_err() {
        return Err(String::from("Error: Unable to parse viewbox width. Malformed Data. "));
    } else if height.is_err() {
        return Err(String::from("Error: Unable to parse viewbox height. Malformed Data. "));
    }

    Ok((width.unwrap(), height.unwrap()))
}