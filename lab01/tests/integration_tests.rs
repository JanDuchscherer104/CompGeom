use lab01::{geometry::Line2D, utils::read_lines_from_file};

#[test]
fn identical_number_of_intersections_for_internal_and_external_computation() {
    let lines = read_lines_from_file("./.data/s_1000_1.dat").unwrap();

    let internal_intersections = lab01::naive_intersect::get_intersections(&lines);
    let external_intersections = lab01::external_library::get_intersections_external(&lines);

    for intersection in internal_intersections.iter() {
        println!("(({}, {}),({}, {})) x (({}, {}), ({},{}))", intersection.0.start.x, intersection.0.start.y, intersection.0.end.x, intersection.0.end.y, intersection.1.start.x, intersection.1.start.y, intersection.1.end.x, intersection.1.end.y);
    }

    assert_eq!(internal_intersections.len(), external_intersections.len());
}
