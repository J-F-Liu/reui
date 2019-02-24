use reui::properties::{Size, Text};
use reui::widgets::{Pane, TextBlock};
use reui::Widget;

#[test]
fn test_pane() {
    let elem = Pane {
        top: 0.0,
        left: 0.0,
        width: 10.0,
        height: 20.0,
        children: vec![],
    }
    .render();

    assert_eq!(elem.property::<Size>(), Some(&Size::new(10.0, 20.0)));
}
#[test]
fn test_text() {
    let elem = TextBlock { text: "TextBlock" }.render();

    assert_eq!(
        elem.property_clone::<Text>().map(|t| t.0),
        Some("TextBlock".to_owned())
    );
}
