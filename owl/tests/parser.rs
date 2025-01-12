use owl::*;

#[test]
fn parse_something() {
    let _: Document = "Hello World".parse().unwrap();
}

#[test]
fn parse_heading() {
    let document: Document = "
# Hello World
".parse().unwrap();
    assert_eq!(document.parts.len(), 1);
    match document.parts[0] {
        Part::Section(_) => (),
        _ => panic!("expected Section found {:?}", document.parts[0]),
    }
}

