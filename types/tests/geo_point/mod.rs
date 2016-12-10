pub mod mapping;
pub mod formats;

use georust::{ Geometry, ToGeo, Point, Coordinate };

use elastic_types::geo::point::prelude::*;

#[test]
fn can_change_point_mapping() {
	fn takes_custom_mapping(_: GeoPoint<GeoPointObject>) -> bool {
		true
	}

	let point: GeoPoint<GeoPointString> = GeoPoint::new(Point(Coordinate { x: 1.0, y: 1.0 }));

	assert!(takes_custom_mapping(point.remap()));
}

#[test]
fn can_build_point_from_geo() {
	let coord = Coordinate { x: 1.0, y: 1.0 };

	let point = GeoPoint::<DefaultGeoPointFormat>::new(Point(coord.clone()));

	assert_eq!(
		(coord.x, coord.y),
		(point.x(), point.y())
	);
}

#[test]
fn can_convert_point_to_geo() {
	let point = GeoPoint::<DefaultGeoPointFormat>::new(Point(Coordinate { x: 1.0, y: 1.0 }));
	let geo = point.to_geo();

	match geo {
		Geometry::Point(point) => assert_eq!(
			(1.0, 1.0),
			(point.x(), point.y())
		),
		_ => panic!("expected point")
	}
}
