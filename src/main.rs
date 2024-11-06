use rstar::{Point, RTree};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, PartialEq, Debug)]
struct TimeZone {
	lat: f64,
	lng: f64,
	name: Option<String>,
}

struct CSVLine<'a> {
	line: &'a [u8],
	pos: usize
}
impl<'a> CSVLine<'a> {
	fn new(line: &'a str) -> Self {
		Self { line: line.as_bytes(), pos: 0 }
	}
}
impl<'a> Iterator for CSVLine<'a> { //doesnt work well when line ends in comma
	type Item = &'a str;
	fn next(&mut self) -> Option<Self::Item> {
		match self.line.into_iter().nth(self.pos) {
			Some(b'"') => {
				let begin = self.pos+1;
				let end = begin + self.line[begin..].windows(2).position(|w| w == b"\",").unwrap();
				let res = Some(std::str::from_utf8(&self.line[begin..end]).unwrap());
				self.pos = end + 2;
				res
			}
			Some(_) => {
				let end = self.line[self.pos..].iter().position(|&x| x == b',').map(|x| x + self.pos).unwrap_or(self.line.len());
				let res = Some(std::str::from_utf8(&self.line[self.pos..end]).unwrap());
				self.pos = end + 1;
				res
			},
			None => None,
		}
	}
}

impl Point for TimeZone {
	type Scalar = f64;
	const DIMENSIONS: usize = 2;

	fn generate(mut generator: impl FnMut(usize) -> Self::Scalar) -> Self {
		TimeZone {
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
	let mut tree: RTree<TimeZone> = RTree::new();
	let tzfile = File::open("geolite2-city-ipv4-num.csv")?;
	let reader = BufReader::new(tzfile);
	for line_result in reader.lines() {
		let line_string = line_result?;
		let line: CSVLine = CSVLine::new(line_string.as_str());
		let mut iter = line.into_iter().skip(7);
		let lat = iter.next().unwrap();
		let lng = iter.next().unwrap();
		let tz = iter.next().unwrap();
		tree.insert(TimeZone {
			lat: lat.parse::<f64>()?,
			lng: lng.parse::<f64>()?,
			name: Some(tz.to_string()),
		});
	}

	let dbip_file = File::open("dbip-city-ipv4-num.csv")?;
	let reader = BufReader::new(dbip_file);
	for line_result in reader.lines() {
		let line_string = line_result?;
		assert!(line_string.chars().last().unwrap() == ','); //assert empty time zone
		let line = CSVLine::new(line_string.as_str());
		let mut iter = line.into_iter().skip(7);
		let lat = iter.next().unwrap();
		let lng = iter.next().unwrap();
		let lat = lat.parse::<f64>()?;
		let lng = lng.parse::<f64>()?;
		let tz = tree.nearest_neighbor(
			&TimeZone {
				lat,
				lng,
				name: None,
			}
		).unwrap().name.clone().unwrap();
		println!("{}{}",line_string,tz);
	}

	Ok(())
}
