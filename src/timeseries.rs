use std::ops::Index;
use std::thread::sleep;
use std::option::Option;
use std::cmp::PartialEq;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive (Debug)]
pub struct Record {
    timestamp: u128,
    value: f64,
}

impl PartialEq for Record {
    fn eq(&self, r: &Record) -> bool {
        return self.value == r.value && self.timestamp == r.timestamp
    }
}

impl Record {
    pub fn new(value: f64) -> Record {
        let ctime = SystemTime::now().duration_since(UNIX_EPOCH).expect("Unable to get now");
        Record {
            timestamp: ctime.as_millis(),
            value: value
        }
    }
}

pub struct TimeSeries {
    name: String,
    retention: i64,
    ctime: u128,
    records: Vec<Record>
}

impl Index<usize> for TimeSeries {

    type Output = Record;

    fn index(&self, i: usize) -> &Record {
        return &self.records[i];
    }
}

impl TimeSeries {

    pub fn new(name: String, retention: i64) -> TimeSeries {
        let ctime = SystemTime::now().duration_since(UNIX_EPOCH).expect("Unable to get now");
        TimeSeries {
            name: name,
            retention: retention,
            ctime: ctime.as_millis(),
            records: Vec::new()
        }
    }

    pub fn add_point(&mut self, r: Record) {
        self.records.push(r);
    }

    pub fn avg(&self) -> f64 {
        let a: f64 = self.records
            .iter()
            .map(|x| x.value)
            .sum::<f64>() / self.records.len() as f64;
        return a;
    }

    pub fn avg_interval(&self, interval: u128) -> Option<Vec<f64>> {
        match self.records.first() {
            Some(first) => {
                let first_ts = (first.timestamp / interval) * interval;
                let last = self.records.last().unwrap();
                let last_ts = ((last.timestamp / interval) * interval) + interval;
                let mut current_ts = first_ts + interval;
                let mut avgs: Vec<f64> = Vec::new();
                while current_ts <= last_ts {
                    let range: Vec<f64> = self.records
                        .iter()
                        .filter(|v| v.timestamp > current_ts-interval
                            && v.timestamp < current_ts)
                        .map(|x| x.value).collect();
                    if range.len() > 0 {
                        avgs.push(
                            range.iter().sum::<f64>() / range.len() as f64
                        );
                    }
                    current_ts += interval;
                }
                return Some(avgs);
            },
            None => return None
        };
    }

    pub fn len(&self) -> usize {
        return self.records.len();
    }

    pub fn is_empty(&self) -> bool {
        return self.records.len() == 0
    }

    pub fn max(&self) -> Option<f64> {
        if self.is_empty() {
            return None;
        }
        let first = self.records[0].value;
        return Some(
            self.records
            .iter()
            .map(|x| x.value)
            .fold(first, |max, val| if val > max { val } else { max })
        );
    }

    pub fn min(&self) -> Option<f64> {
        if self.is_empty() {
            return None;
        }
        let first = self.records[0].value;
        return Some(
            self.records
            .iter()
            .map(|x| x.value)
            .fold(first, |min, val| if min < val { min } else { val })
        );
    }

    pub fn search(&self, val: u128) -> Result<usize, usize> {
        return self.records.binary_search_by(|r| r.timestamp.cmp(&val));
    }
}

//////////////////////
///   UNIT TESTS   ///
//////////////////////

#[test]
fn test_ts_new() {
    let ts = TimeSeries::new("test-ts".to_string(), 3000);
    assert_eq!(ts.name, "test-ts");
    assert_eq!(ts.retention, 3000);
}

#[test]
fn test_ts_add_point() {
    let mut ts = TimeSeries::new("test-ts".to_string(), 0);
    let r = Record::new(12.98);
    ts.add_point(r);
    assert_eq!(ts.records.len(), 1);
    assert_eq!(ts.records[0].value, 12.98);
}

#[test]
fn test_ts_avg() {
    let mut ts = TimeSeries::new("test-ts".to_string(), 0);
    let r1 = Record::new(12.98);
    let r2 = Record::new(19.63);
    let r3 = Record::new(11.28);
    let r4 = Record::new(15.96);
    ts.add_point(r1);
    ts.add_point(r2);
    ts.add_point(r3);
    ts.add_point(r4);
    let avg = ts.avg();
    assert_eq!(avg, 14.9625);
}

#[test]
fn test_ts_avg_interval() {
    let mut ts = TimeSeries::new("test-ts".to_string(), 0);
    let r1 = Record::new(12.98);
    sleep(Duration::new(0, 5e8 as u32));
    let r2 = Record::new(19.63);
    let r3 = Record::new(11.28);
    sleep(Duration::new(0, 5e8 as u32));
    let r4 = Record::new(15.96);
    ts.add_point(r1);
    ts.add_point(r2);
    ts.add_point(r3);
    ts.add_point(r4);
    let avg = ts.avg_interval(500 as u128).unwrap();
    assert_eq!(avg, [12.98, 15.454999999999998, 15.96]);
}

#[test]
fn test_ts_index() {
    let mut ts = TimeSeries::new("test-ts".to_string(), 0);
    let r1 = Record::new(12.98);
    let r2 = Record::new(19.63);
    let r3 = Record::new(11.28);
    let r4 = Record::new(15.96);
    ts.add_point(r1);
    ts.add_point(r2);
    ts.add_point(r3);
    ts.add_point(r4);
    assert_eq!(ts[1].value, 19.63);
    assert_eq!(ts[3].value, 15.96);
}

#[test]
fn test_ts_len() {
    let mut ts = TimeSeries::new("test-ts".to_string(), 0);
    assert_eq!(ts.len(), 0);
    let r1 = Record::new(12.98);
    ts.add_point(r1);
    assert_eq!(ts.len(), 1);
}

#[test]
fn test_ts_is_empty() {
    let mut ts = TimeSeries::new("test-ts".to_string(), 0);
    assert_eq!(ts.is_empty(), true);
    let r1 = Record::new(12.98);
    ts.add_point(r1);
    assert_eq!(ts.is_empty(), false);
}

#[test]
fn test_ts_max() {
    let mut ts = TimeSeries::new("test-ts".to_string(), 0);
    let r1 = Record::new(12.98);
    let r2 = Record::new(19.63);
    let r3 = Record::new(11.28);
    let r4 = Record::new(15.96);
    ts.add_point(r1);
    ts.add_point(r2);
    ts.add_point(r3);
    ts.add_point(r4);
    assert_eq!(ts.max(), Some(19.63));
}

#[test]
fn test_ts_min() {
    let mut ts = TimeSeries::new("test-ts".to_string(), 0);
    let r1 = Record::new(12.98);
    let r2 = Record::new(19.63);
    let r3 = Record::new(11.28);
    let r4 = Record::new(15.96);
    ts.add_point(r1);
    ts.add_point(r2);
    ts.add_point(r3);
    ts.add_point(r4);
    assert_eq!(ts.min(), Some(11.28));
}

#[test]
fn test_ts_search() {
    let mut ts = TimeSeries::new("test-ts".to_string(), 0);
    let r1 = Record::new(12.98);
    sleep(Duration::new(0, 5e8 as u32));
    let r2 = Record::new(19.63);
    let r3 = Record::new(11.28);
    sleep(Duration::new(0, 5e8 as u32));
    let r4 = Record::new(15.96);
    let timestamp_1 = r2.timestamp;
    let timestamp_2 = timestamp_1 + 10;
    ts.add_point(r1);
    ts.add_point(r2);
    ts.add_point(r3);
    ts.add_point(r4);
    assert_eq!(ts.search(timestamp_1), Ok(2));
    assert_eq!(ts.search(timestamp_2), Err(3));
}

#[test]
fn test_record_new() {
    let r = Record::new(12.98);
    assert_eq!(r.value, 12.98);
}
