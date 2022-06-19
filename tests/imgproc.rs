#![cfg(ocvrs_has_module_imgproc)]

use opencv::{
	core::{Point, Point2f, Scalar, Size, Vec2f},
	imgproc,
	prelude::*,
	Result,
	types::VectorOfPoint,
};

#[test]
fn min_enclosing() -> Result<()> {
	let mut pts = Mat::new_rows_cols_with_default(1, 2, Vec2f::typ(), Scalar::default())?;
	let points = pts.at_row_mut::<Vec2f>(0)?;
	points[0].copy_from_slice(&[10., 10.]);
	points[1].copy_from_slice(&[20., 10.]);

	let mut center = Point2f::default();
	let mut radius = 0.;
	imgproc::min_enclosing_circle(&pts, &mut center, &mut radius)?;
	assert_eq!(radius, 5.0001);
	assert_eq!(center, Point2f::new(15., 10.));
	Ok(())
}

#[test]
fn ellipse() -> Result<()> {
	let mut pts = VectorOfPoint::new();
	imgproc::ellipse_2_poly(Point::new(100, 100), Size::new(200, 200), 0, 45, 90, 10, &mut pts)?;
	assert_eq!(6, pts.len());
	assert_eq!(Point::new(241, 241), pts.get(0)?);
	assert_eq!(Point::new(100, 300), pts.get(5)?);
	Ok(())
}

#[test]
fn get_rotation_matrix_2d() -> Result<()> {
	let mat = imgproc::get_rotation_matrix_2d(Point2f::new(10., 10.), 90., 2.)?;
	assert_eq!(Size::new(3, 2), mat.size()?);
	assert_eq!(*mat.at_2d::<f64>(0, 0)?, *mat.at_2d::<f64>(1, 1)?);
	assert_eq!(-*mat.at_2d::<f64>(0, 1)?, *mat.at_2d::<f64>(1, 0)?);
	Ok(())
}
