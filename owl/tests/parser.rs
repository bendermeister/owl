use owl::*;

#[test]
fn unexpected_heading_level_1() {
    let result = "## Some heading".parse::<Document>();

    let expected = ParsingError::UnexpectedHeadingLevel {
        line_number: 1,
        expected: 1,
        got: 2,
    };

    if let Err(err) = result {
        assert_eq!(expected, err);
    } else {
        panic!(
            "Expected: '{:?}' got: '{:?}'",
            Err::<Document, ParsingError>(expected),
            result
        );
    }
}

#[test]
fn unexpected_heading_level_2() {
    let result = "# First Heading
### Some Heading
"
    .parse::<Document>();

    let expected = ParsingError::UnexpectedHeadingLevel {
        line_number: 2,
        expected: 2,
        got: 3,
    };

    if let Err(err) = result {
        assert_eq!(expected, err, "error");
    } else {
        panic!(
            "Expected: '{:?}' got: '{:?}'",
            Err::<Document, ParsingError>(expected),
            result
        );
    }
}

#[test]
fn unexpected_heading_level_3() {
    let doc = "# First Heading
## First SubHeading
### First SubSubHeading
## Second SubHeading"
        .parse::<Document>()
        .unwrap();
    assert_eq!(doc.blocks.len(), 4);

    let expected = Block::Heading{ level: 1, body: "First Heading".into() };
    assert_eq!(expected, doc.blocks[0]);

    let expected = Block::Heading{ level: 2, body: "First SubHeading".into() };
    assert_eq!(expected, doc.blocks[1]);

    let expected = Block::Heading{ level: 3, body: "First SubSubHeading".into() };
    assert_eq!(expected, doc.blocks[2]);

    let expected = Block::Heading{ level: 2, body: "Second SubHeading".into() };
    assert_eq!(expected, doc.blocks[3]);
}

#[test]
fn parse_heading_leading_zeroes_error() {
    let result = " # Hello World".parse::<Document>();

    if let Err(ParsingError::LeadingWhiteSpaceBeforeHeading { line_number }) = result {
        assert_eq!(line_number, 1, "line_number");
    } else {
        panic!(
            "Expected: '{:?}' got: {:?}",
            ParsingError::LeadingWhiteSpaceBeforeHeading { line_number: 0 },
            result
        );
    }
}

#[test]
fn parse_heading_leading_zeroes_error_2() {
    let doc = "# Hello World
 # This should fail"
        .parse::<Document>();

    if let Err(ParsingError::LeadingWhiteSpaceBeforeHeading { line_number }) = doc {
        assert_eq!(line_number, 2, "line_number");
    } else {
        panic!("Expected Error got Ok: {:?}", doc);
    }
}

#[test]
fn parse_heading_level_1() {
    let doc: Document = "# Hello World".parse().unwrap();
    assert_eq!(doc.blocks.len(), 1);

    let expected = Block::Heading{ level: 1, body: "Hello World".into() };
    assert_eq!(expected, doc.blocks[0]);
}

#[test]
fn parse_2_heading_level_1() {
    let doc: Document = "# Hello World
# Bye World
"
    .parse()
    .unwrap();
    assert_eq!(doc.blocks.len(), 2);

    let expected = Block::Heading{ level: 1, body: "Hello World".into() };
    assert_eq!(expected, doc.blocks[0]);

    let expected = Block::Heading{ level: 1, body: "Bye World".into() };
    assert_eq!(expected, doc.blocks[1]);
}

#[test]
fn parse_heading_level_2() {
    let doc: Document = "# First Heading
## Second Heading"
        .parse()
        .unwrap();
    assert_eq!(doc.blocks.len(), 2);

    let expected = Block::Heading{ level: 1, body: "First Heading".into() };
    assert_eq!(expected, doc.blocks[0]);

    let expected = Block::Heading{ level: 2, body: "Second Heading".into() };
    assert_eq!(expected, doc.blocks[1]);
}
