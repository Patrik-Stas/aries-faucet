use bson::Document;

pub fn build_label_filter(label: Option<String>) -> Document {
    let mut filter: Document = doc! { };
    match label {
        Some(label) => {
            filter.insert(
                "label",
                doc! {
                    "$eq": label
                },
            );
        }
        _ => {}
    };
    filter
}