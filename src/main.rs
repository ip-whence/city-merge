use rstar::{Point, RTree};
use std::fs::File;
use std::io::{BufRead, BufReader};
use chrono_tz::{Tz, UTC};
use chrono::{Datelike, TimeZone, LocalResult::{Single}};

#[derive(Clone, PartialEq, Debug)]
struct PtTimeZone {
	lat: f64,
	lng: f64,
	name: Option<String>,
}

impl Point for PtTimeZone {
	type Scalar = f64;
	const DIMENSIONS: usize = 2;

	fn generate(mut generator: impl FnMut(usize) -> Self::Scalar) -> Self {
		PtTimeZone {
			lat: generator(0),
			lng: generator(1),
			name: None,
		}
	}

	fn nth(&self, index: usize) -> Self::Scalar {
		match index {
			0 => self.lat,
			1 => self.lng,
			_ => unreachable!(),
		}
	}

	fn nth_mut(&mut self, index: usize) -> &mut Self::Scalar {
		match index {
			0 => &mut self.lat,
			1 => &mut self.lng,
			_ => unreachable!(),
		}
	}
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let mut tree: RTree<PtTimeZone> = RTree::new();
	let tzfile = File::open("geolite2-city-ipv4-num.csv")?;
	let reader = BufReader::new(tzfile);
	for line_result in reader.lines() {
		let line_string = line_result?;
		let mut ss = line_string.split(',').rev();
		let tz = ss.next().unwrap();
		let lng = ss.next().unwrap();
		let lat = ss.next().unwrap();
		tree.insert(PtTimeZone {
			lat: lat.parse::<f64>()?,
			lng: lng.parse::<f64>()?,
			name: Some(tz.to_string()),
		});
	}

	let dbip_file = File::open("dbip-city-ipv4-num.csv")?;
	let reader = BufReader::new(dbip_file);

	let current_date = chrono::Utc::now();
	let year = current_date.year();
	let month = current_date.month();
	let day = current_date.day();
	for line_result in reader.lines() {
		let line_string = line_result?;
		let mut ss = line_string.split(',').rev();
		assert!(ss.next().unwrap().is_empty());
		let lng = ss.next().unwrap();
		let lat = ss.next().unwrap();
		let tz_name = tree.nearest_neighbor(
			&PtTimeZone {
				lat: lat.parse::<f64>()?,
				lng: lng.parse::<f64>()?,
				name: None,
			}
		).unwrap().name.clone().unwrap();

		let tz: Tz = tz_name.parse().unwrap();
		let dt = tz.with_ymd_and_hms(year, month, day, 12, 0, 0);
    let utc = UTC.with_ymd_and_hms(year, month, day, 12, 0, 0);
		let tz_minutes_offset = match (dt, utc) {
			(Single(dts), Single(utcs)) => utcs - dts,
			_ => panic!("Ambiguous time"),
		}.num_minutes();
		println!("{}{},{}",line_string,tz_name,tz_minutes_offset);
	}

	Ok(())
}
