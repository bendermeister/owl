//use owl::*;

// // TODO: reset item_level tracking in parser
// #[test]
// fn item_unexpected_level_interrupted() {
//     let result = "# Some List
// - List 1
//   - List 2
//     - List 3
//     - List 3
// ## Some Heading
//   - List ERROR
// "
//     .parse::<Document>();
//     let expected = ParsingError::ItemUnexpectedLevel {
//         line_number: 7,
//         expected: 1,
//         got: 2,
//     };
//
//     if let Err(gotten) = result {
//         assert_eq!(expected, gotten);
//     } else {
//         panic!("Expected: `{:?}`, gotten: `{:?}`", expected, result);
//     }
// }
//
// #[test]
// fn item_leading_odd_space() {
//     let result = "# Some List
// - Item 1
//  - Item 2"
//         .parse::<Document>();
//
//     let expected = ParsingError::ItemOddLeadingSpace { line_number: 3 };
//
//     if let Err(gotten) = result {
//         assert_eq!(expected, gotten);
//     } else {
//         panic!("Expected: `{:?}`, gotten: `{:?}`", expected, result);
//     }
// }
//
// #[test]
// fn item_unexpected_level_nested() {
//     let result = "# Some List
// - Level 1
// - Level 1
//     - Level 3"
//         .parse::<Document>();
//
//     let expected = ParsingError::ItemUnexpectedLevel {
//         line_number: 4,
//         expected: 2,
//         got: 3,
//     };
//
//     if let Err(gotten) = result {
//         assert_eq!(expected, gotten);
//     } else {
//         panic!("Expected: `{:?}`, gotten: `{:?}`", expected, result);
//     }
// }
//
// #[test]
// fn item_unexpected_level() {
//     let result = "# Some List
//   - Level 2"
//         .parse::<Document>();
//
//     let expected = ParsingError::ItemUnexpectedLevel {
//         line_number: 2,
//         expected: 1,
//         got: 2,
//     };
//
//     if let Err(gotten) = result {
//         assert_eq!(expected, gotten);
//     } else {
//         panic!("Expected: `{:?}`, gotten: `{:?}`", expected, result);
//     }
// }
//
// #[test]
// fn item_sublevel() {
//     let doc: Document = "# Some List
// - Level 1/1
//   - Level 2/1
// - Level 1/2
//   - Level 2/2
//   - Level 2/3
//     More of Level 2/3
// "
//     .parse()
//     .unwrap();
//
//     assert_eq!(6, doc.blocks.len());
//
//     let mut blocks = doc.blocks.iter();
//
//     let expected = Line::Heading {
//         level: 1,
//         body: "Some List".into(),
//     };
//     assert_eq!(expected, *blocks.next().unwrap());
//
//     let expected = Line::Item {
//         level: 1,
//         body: "Level 1/1".into(),
//     };
//     assert_eq!(expected, *blocks.next().unwrap());
//
//     let expected = Line::Item {
//         level: 2,
//         body: "Level 2/1".into(),
//     };
//     assert_eq!(expected, *blocks.next().unwrap());
//
//     let expected = Line::Item {
//         level: 1,
//         body: "Level 1/2".into(),
//     };
//     assert_eq!(expected, *blocks.next().unwrap());
//
//     let expected = Line::Item {
//         level: 2,
//         body: "Level 2/2".into(),
//     };
//     assert_eq!(expected, *blocks.next().unwrap());
//
//     let expected = Line::Item {
//         level: 2,
//         body: "Level 2/3 More of Level 2/3".into(),
//     };
//     assert_eq!(expected, *blocks.next().unwrap());
// }
//
// #[test]
// fn item_with_block() {
//     let doc: Document = "# Some List
// - First Item
// - Second Item
//   Second Part of Second Item
//   Third Part of Second Item"
//         .parse()
//         .unwrap();
//
//     assert_eq!(3, doc.blocks.len());
//
//     let expected = Line::Heading {
//         level: 1,
//         body: "Some List".into(),
//     };
//     assert_eq!(expected, doc.blocks[0]);
//
//     let expected = Line::Item {
//         level: 1,
//         body: "First Item".into(),
//     };
//     assert_eq!(expected, doc.blocks[1]);
//
//     let expected = Line::Item {
//         level: 1,
//         body: "Second Item Second Part of Second Item Third Part of Second Item".into(),
//     };
//     assert_eq!(expected, doc.blocks[2]);
// }
//
// #[test]
// fn item_basic() {
//     let doc: Document = "# Some List
// - First Element
// - Second Element
// - Third Element"
//         .parse()
//         .unwrap();
//
//     assert_eq!(doc.blocks.len(), 4, "blocks.len()");
//
//     let expected = Line::Heading {
//         level: 1,
//         body: "Some List".into(),
//     };
//     assert_eq!(expected, doc.blocks[0], "Heading");
//
//     let expected = Line::Item {
//         level: 1,
//         body: "First Element".into(),
//     };
//     assert_eq!(expected, doc.blocks[1]);
//
//     let expected = Line::Item {
//         level: 1,
//         body: "Second Element".into(),
//     };
//     assert_eq!(expected, doc.blocks[2]);
//
//     let expected = Line::Item {
//         level: 1,
//         body: "Third Element".into(),
//     };
//     assert_eq!(expected, doc.blocks[3]);
// }
//
// #[test]
// fn unexpected_heading_level_1() {
//     let result = "## Some heading".parse::<Document>();
//
//     let expected = ParsingError::HeadingUnexpectedLevel {
//         line_number: 1,
//         expected: 1,
//         got: 2,
//     };
//
//     if let Err(err) = result {
//         assert_eq!(expected, err);
//     } else {
//         panic!(
//             "Expected: '{:?}' got: '{:?}'",
//             Err::<Document, ParsingError>(expected),
//             result
//         );
//     }
// }
//
// #[test]
// fn heading_1() {
//     let doc = "# Level 1
// ## Level 2
// ## Level 2/2"
//         .parse::<Document>()
//         .unwrap();
//
//     assert_eq!(doc.blocks.len(), 3);
//
//     let expected = Line::Heading {
//         level: 1,
//         body: "Level 1".into(),
//     };
//     assert_eq!(expected, doc.blocks[0]);
//
//     let expected = Line::Heading {
//         level: 2,
//         body: "Level 2".into(),
//     };
//     assert_eq!(expected, doc.blocks[1]);
//
//     let expected = Line::Heading {
//         level: 2,
//         body: "Level 2/2".into(),
//     };
//     assert_eq!(expected, doc.blocks[2]);
// }
//
// #[test]
// fn heading_with_permitted_space() {
//     let doc = "# Level 1
// ## Level 2
//
// ## Level 2
//
// ### Level 3
// ### Level 3
// "
//     .parse::<Document>()
//     .unwrap();
//
//     assert_eq!(doc.blocks.len(), 5, "blocks.len()");
//
//     let expected = Line::Heading {
//         level: 1,
//         body: "Level 1".into(),
//     };
//     assert_eq!(expected, doc.blocks[0]);
//
//     let expected = Line::Heading {
//         level: 2,
//         body: "Level 2".into(),
//     };
//     assert_eq!(expected, doc.blocks[1]);
//
//     let expected = Line::Heading {
//         level: 2,
//         body: "Level 2".into(),
//     };
//     assert_eq!(expected, doc.blocks[2]);
//
//     let expected = Line::Heading {
//         level: 3,
//         body: "Level 3".into(),
//     };
//     assert_eq!(expected, doc.blocks[3]);
//
//     let expected = Line::Heading {
//         level: 3,
//         body: "Level 3".into(),
//     };
//     assert_eq!(expected, doc.blocks[4]);
// }
//
// #[test]
// fn unexpected_heading_level_2() {
//     let result = "# First Heading
// ### Some Heading
// "
//     .parse::<Document>();
//
//     let expected = ParsingError::HeadingUnexpectedLevel {
//         line_number: 2,
//         expected: 2,
//         got: 3,
//     };
//
//     if let Err(err) = result {
//         assert_eq!(expected, err, "error");
//     } else {
//         panic!(
//             "Expected: '{:?}' got: '{:?}'",
//             Err::<Document, ParsingError>(expected),
//             result
//         );
//     }
// }
//
// #[test]
// fn unexpected_heading_level_3() {
//     let doc = "# First Heading
// ## First SubHeading
// ### First SubSubHeading
// ## Second SubHeading"
//         .parse::<Document>()
//         .unwrap();
//     assert_eq!(doc.blocks.len(), 4);
//
//     let expected = Line::Heading {
//         level: 1,
//         body: "First Heading".into(),
//     };
//     assert_eq!(expected, doc.blocks[0]);
//
//     let expected = Line::Heading {
//         level: 2,
//         body: "First SubHeading".into(),
//     };
//     assert_eq!(expected, doc.blocks[1]);
//
//     let expected = Line::Heading {
//         level: 3,
//         body: "First SubSubHeading".into(),
//     };
//     assert_eq!(expected, doc.blocks[2]);
//
//     let expected = Line::Heading {
//         level: 2,
//         body: "Second SubHeading".into(),
//     };
//     assert_eq!(expected, doc.blocks[3]);
// }
//
// #[test]
// fn parse_heading_leading_zeroes_error() {
//     let result = " # Hello World".parse::<Document>();
//
//     if let Err(ParsingError::HeadingLeadingWhiteSpace { line_number }) = result {
//         assert_eq!(line_number, 1, "line_number");
//     } else {
//         panic!(
//             "Expected: '{:?}' got: {:?}",
//             ParsingError::HeadingLeadingWhiteSpace { line_number: 0 },
//             result
//         );
//     }
// }
//
// #[test]
// fn parse_heading_leading_zeroes_error_2() {
//     let doc = "# Hello World
//  # This should fail"
//         .parse::<Document>();
//
//     if let Err(ParsingError::HeadingLeadingWhiteSpace { line_number }) = doc {
//         assert_eq!(line_number, 2, "line_number");
//     } else {
//         panic!("Expected Error got Ok: {:?}", doc);
//     }
// }
//
// #[test]
// fn parse_heading_level_1() {
//     let doc: Document = "# Hello World".parse().unwrap();
//     assert_eq!(doc.blocks.len(), 1);
//
//     let expected = Line::Heading {
//         level: 1,
//         body: "Hello World".into(),
//     };
//     assert_eq!(expected, doc.blocks[0]);
// }
//
// #[test]
// fn parse_2_heading_level_1() {
//     let result = "# Hello World
// # Bye World
// "
//     .parse::<Document>();
//
//     let expected = ParsingError::HeadingMultipleRoots { line_number: 2 };
//     if let Err(gotten) = result {
//         assert_eq!(expected, gotten);
//     } else {
//         panic!("Expected: '{:?}', gotten: '{:?}'", expected, result);
//     }
// }
//
// #[test]
// fn parse_heading_level_2() {
//     let doc: Document = "# First Heading
// ## Second Heading"
//         .parse()
//         .unwrap();
//     assert_eq!(doc.blocks.len(), 2);
//
//     let expected = Line::Heading {
//         level: 1,
//         body: "First Heading".into(),
//     };
//     assert_eq!(expected, doc.blocks[0]);
//
//     let expected = Line::Heading {
//         level: 2,
//         body: "Second Heading".into(),
//     };
//     assert_eq!(expected, doc.blocks[1]);
// }
