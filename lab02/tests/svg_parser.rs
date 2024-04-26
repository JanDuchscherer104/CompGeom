use lab02::{point::Point2D, svg_parser::parse_file_into_country};

#[test]
fn test_parse_file_into_country_should_return_a_vector_of_states() {
    let path = String::from("tests/test.svg");
    let states = parse_file_into_country(path).states;

    assert_eq!(states.len(), 3);
}

#[test]
fn test_parse_file_into_country_should_return_a_vector_of_states_with_correct_names() {
    let path = String::from("tests/test.svg");
    let states = parse_file_into_country(path).states;

    assert_eq!(states[0].name, "RelativePath");
    assert_eq!(states[1].name, "AbsolutePath");
    assert_eq!(states[2].name, "StateWithMultiplePolygons");
}

#[test]
fn test_parse_file_into_country_should_return_states_with_correct_polygons() {
    let path = String::from("tests/test.svg");
    let states = parse_file_into_country(path).states;

    assert_eq!(states[0].polygons.len(), 1);
    assert_eq!(states[1].polygons.len(), 1);
    assert_eq!(states[2].polygons.len(), 2);
}

#[test]
fn test_parse_file_into_country_should_parse_relative_path_correctly() {
    let path = String::from("tests/test.svg");
    let states = parse_file_into_country(path).states;

    let state = &states[0];
    let polygon = &state.polygons[0];

    // M 0, 0
    // l 100, 0
    // l 0, 100
    // l -100, 0
    // l 0, -100
    // z
    let expected = vec![
        Point2D::new(0.0, 0.0),
        Point2D::new(100.0, 0.0),
        Point2D::new(100.0, 100.0),
        Point2D::new(0.0, 100.0),
    ];

    assert_eq!(polygon.points.len(), expected.len());
    for (i, point) in polygon.points.iter().enumerate() {
        assert!(
            point.approx_eq(&expected[i]),
            "The point {} is not equal to the expected point {}",
            point,
            expected[i]
        );
    }
}

#[test]
fn test_parse_file_into_country_should_parse_absolute_path_correctly() {
    let path = String::from("tests/test.svg");
    let states = parse_file_into_country(path).states;

    let state = &states[1];
    let polygon = &state.polygons[0];

    // M 0, 300
    // L 0, 400
    // L 100, 400
    // L 100, 300
    // l 0, 300
    // z
    let expected = vec![
        Point2D::new(0.0, 300.0),
        Point2D::new(0.0, 400.0),
        Point2D::new(100.0, 400.0),
        Point2D::new(100.0, 300.0),
    ];

    assert_eq!(polygon.points.len(), expected.len());
    for (i, point) in polygon.points.iter().enumerate() {
        assert!(
            point.approx_eq(&expected[i]),
            "The point {} is not equal to the expected point {}",
            point,
            expected[i]
        );
    }
}

#[test]
fn test_parse_file_into_country_should_parse_multiple_polygons_correctly() {
    let path = String::from("tests/test.svg");
    let states = parse_file_into_country(path).states;

    let state = &states[2];
    let polygon0 = &state.polygons[0];
    let polygon1 = &state.polygons[1];

    // M 200, 0
    // l 100, 100
    // l 100, 0
    // l -100, 100
    // l -100, 0
    // l 0, -200
    // z
    // M 300, 0
    // l 50, 0
    // l 0, 50
    // l -50, 0
    // l 0, -50
    // z
    let expected0 = vec![
        Point2D::new(200.0, 0.0),
        Point2D::new(300.0, 100.0),
        Point2D::new(400.0, 100.0),
        Point2D::new(300.0, 200.0),
        Point2D::new(200.0, 200.0),
    ];
    let expected1 = vec![
        Point2D::new(300.0, 0.0),
        Point2D::new(350.0, 0.0),
        Point2D::new(350.0, 50.0),
        Point2D::new(300.0, 50.0),
    ];

    assert_eq!(polygon0.points.len(), expected0.len());
    for (i, point) in polygon0.points.iter().enumerate() {
        assert!(
            point.approx_eq(&expected0[i]),
            "The point {} is not equal to the expected point {}",
            point,
            expected0[i]
        );
    }
    assert_eq!(polygon1.points.len(), expected1.len());
    for (i, point) in polygon1.points.iter().enumerate() {
        assert!(
            point.approx_eq(&expected1[i]),
            "The point {} is not equal to the expected point {}",
            point,
            expected1[i]
        );
    }
}

#[test]
fn test_parse_file_into_country_should_return_cities() {
    let path = String::from("tests/test.svg");
    let cities = parse_file_into_country(path).cities;

    assert_eq!(cities.len(), 2);
    
    assert_eq!(cities[0].name, "City0");
    assert!(cities[0].location.approx_eq(&Point2D::new(50.0, 50.0)));
    assert_eq!(cities[1].name, "City1");
    assert!(cities[1].location.approx_eq(&Point2D::new(250.0, 150.0)));

}