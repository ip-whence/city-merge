use chrono::{Datelike, LocalResult::Single, TimeZone, Utc};
use chrono_tz::{Tz, UTC};
use csv;
use rstar::{Point, RTree};
use std::fs::File;
use std::net::IpAddr;
use std::str::FromStr;

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

fn ip_to_u128(ip_str: &str) -> Result<u128, Box<dyn std::error::Error>> {
	// Try parsing as a pure numeric value
	if let Ok(num) = ip_str.parse::<u128>() {
		return Ok(num);
	}

	// Otherwise, parse as an IP address and convert to numeric
	let addr: IpAddr = IpAddr::from_str(ip_str)?;
	Ok(match addr {
		IpAddr::V4(a) => {
			let o = a.octets();
			((o[0] as u128) << 24) | ((o[1] as u128) << 16) | ((o[2] as u128) << 8) | (o[3] as u128)
		}
		IpAddr::V6(a) => {
			let o = a.octets();
			let mut value: u128 = 0;
			for b in o {
				value = (value << 8) | (b as u128);
			}
			value
		}
	})
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let mut tree: RTree<PtTimeZone> = RTree::new();

	// Read geolite2-city-ipv4-num.csv
	let tzfile = File::open("geolite2-city-ipv6-num.csv")?;
	let mut rdr = csv::ReaderBuilder::new()
		.has_headers(false)
		.from_reader(tzfile);

	for result in rdr.records() {
		let record = result?;
		let len = record.len();
		if len < 3 {
			continue; // Not enough fields
		}

		let lat_str = &record[len - 3];
		let lng_str = &record[len - 2];
		let tz_str = &record[len - 1];

		let lat = lat_str.parse::<f64>()?;
		let lng = lng_str.parse::<f64>()?;
		let tz_name = tz_str.to_string();

		tree.insert(PtTimeZone {
			lat,
			lng,
			name: Some(tz_name),
		});
	}

	// Read dbip-city-ipv4-num.csv
	let dbip_file = File::open("dbip-city-ipv6.csv")?;
	let mut rdr_dbip = csv::ReaderBuilder::new()
		.has_headers(false)
		.from_reader(dbip_file);

	let current_date = Utc::now();
	let year = current_date.year();
	let month = current_date.month();
	let day = current_date.day();

	for result in rdr_dbip.records() {
		let record = result?;
		let len = record.len();
		if len < 3 {
			continue; // Not enough fields
		}

		let lat_str = &record[len - 3];
		let lng_str = &record[len - 2];
		let lat = lat_str.parse::<f64>()?;
		let lng = lng_str.parse::<f64>()?;

		let nearest = tree
			.nearest_neighbor(&PtTimeZone {
				lat,
				lng,
				name: None,
			})
			.unwrap();

		let tz_name = nearest.name.clone().unwrap();
		let tz: Tz = tz_name.parse().unwrap();

		let dt = tz.with_ymd_and_hms(year, month, day, 12, 0, 0);
		let utc = UTC.with_ymd_and_hms(year, month, day, 12, 0, 0);
		let tz_minutes_offset = match (dt, utc) {
			(Single(dts), Single(utcs)) => utcs - dts,
			_ => panic!("Ambiguous time"),
		}
		.num_minutes();

		let mut output_fields: Vec<String> = record.iter().map(|s| s.to_string()).collect();

		// Convert the first two fields (IP addresses) to numeric
		if output_fields.len() >= 2 {
			let ip1_numeric = ip_to_u128(&output_fields[0])?;
			let ip2_numeric = ip_to_u128(&output_fields[1])?;
			output_fields[0] = ip1_numeric.to_string();
			output_fields[1] = ip2_numeric.to_string();
		}

		// Remove any empty fields
		output_fields.retain(|s| !s.is_empty());

		println!(
			"{},{},{}",
			output_fields.join(","),
			tz_name,
			tz_minutes_offset
		);
	}

	Ok(())
}
