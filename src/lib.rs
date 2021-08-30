use std::fmt;
use std::num;

use quick_xml::events::Event;
use quick_xml::Reader;

const EARTH_RADIUS: f64 = 6371.0;

pub enum Error {
    XmlError(quick_xml::Error),
    ParseError(num::ParseFloatError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::XmlError(e) => e.fmt(f),
            Error::ParseError(e) => e.fmt(f),
        }
    }
}

impl From<quick_xml::Error> for Error {
    fn from(error: quick_xml::Error) -> Self {
        Error::XmlError(error)
    }
}

impl From<num::ParseFloatError> for Error {
    fn from(error: num::ParseFloatError) -> Self {
        Error::ParseError(error)
    }
}

#[derive(Debug)]
pub struct TrackPoint {
    pub latitude: f64,
    pub longitude: f64,
}

impl TrackPoint {
    pub fn new(latitude: f64, longitude: f64) -> TrackPoint {
        TrackPoint {
            latitude,
            longitude,
        }
    }

    pub fn distance(x: &TrackPoint, y: &TrackPoint) -> f64 {
        let d_lat = (y.latitude - x.latitude).to_radians();
        let d_lon = (y.longitude - x.longitude).to_radians();

        let lat1 = x.latitude.to_radians();
        let lat2 = x.latitude.to_radians();

        let a = (d_lat / 2.0).sin().powi(2) + (d_lon / 2.0).sin().powi(2) * lat1.cos() * lat2.cos();
        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

        c * EARTH_RADIUS
    }
}

pub struct Track {
    pub points: Vec<TrackPoint>,
}

impl Track {
    pub fn new() -> Track {
        Track { points: Vec::new() }
    }

    pub fn add(&mut self, p: TrackPoint) {
        self.points.push(p)
    }

    pub fn length(&self) -> f64 {
        let iter1 = self.points.iter();
        let mut iter2 = self.points.iter();

        if iter2.next().is_some() {
            iter1
                .zip(iter2)
                .fold(0.0, |sum, (p1, p2)| sum + TrackPoint::distance(p1, p2))
        } else {
            0.0
        }
    }

    pub fn from_gpx_file(path: &str) -> Result<Track, Error> {
        let mut reader = Reader::from_file(path)?;
        reader.trim_text(true);

        let mut track = Track::new();

        let mut buf = Vec::new();

        loop {
            match reader.read_event(&mut buf)? {
                Event::Start(e) => {
                    if e.name() == b"trkpt" {
                        let mut track_point = TrackPoint::new(0.0, 0.0);

                        for attr in e.attributes() {
                            let attr = attr?;
                            match attr.key {
                                b"lat" => {
                                    track_point.latitude =
                                        attr.unescape_and_decode_value(&reader)?.parse()?
                                }
                                b"lon" => {
                                    track_point.longitude =
                                        attr.unescape_and_decode_value(&reader)?.parse()?
                                }
                                _ => (),
                            }
                        }

                        track.add(track_point);
                    }
                }
                Event::Eof => break,
                _ => (),
            }

            buf.clear();
        }

        Ok(track)
    }
}
