use core::time;
use std::{collections::HashMap, collections::BTreeMap, fs::File};

use std::fs::OpenOptions;

use chrono::prelude::*;
use trust_dns_resolver::proto::Time;

struct DataPoint {
    value_up: u128,
    value_down: u128,
}

pub struct DataCollector where{
    process_rate_data: HashMap<String, BTreeMap<DateTime<Utc>,DataPoint>>,
    connection_rate_data: HashMap<String, BTreeMap<DateTime<Utc>,DataPoint>>,
    remote_address_rate_data: HashMap<String, BTreeMap<DateTime<Utc>,DataPoint>>,
}

impl DataCollector {
    pub fn new() -> Self {
        Self {
            process_rate_data: HashMap::new(),
            connection_rate_data: HashMap::new(),
            remote_address_rate_data: HashMap::new(),
        }
    }

    pub fn open_process_rate_file(&mut self) {
        let file_path = "process_record.csv";
        match File::open(file_path) {
            Ok(file) => {
                let mut reader = csv::Reader::from_reader(file);
                for result in reader.records() {
                    let record = result.unwrap();
                    let timestamp_read = record.get(0).unwrap().parse::<DateTime<Utc>>().unwrap();
                    let process_name = record.get(1).unwrap().to_string();
                    let up_rate = record.get(2).unwrap().parse::<u128>().unwrap();
                    let down_rate = record.get(3).unwrap().parse::<u128>().unwrap();
                    // Store the process name and rate in the process_rate_data HashMap
                    self.process_rate_data
                        .entry(process_name)
                        .or_insert(BTreeMap::new())
                        .insert(timestamp_read, DataPoint {
                            value_up: up_rate,
                            value_down: down_rate,
                        });
                };
            } ,
            Err(_) => {},
        };

    }

    pub fn open_connection_rate_file(&mut self) {
        let file_path = "connection_record.csv";
        match File::open(file_path) {
            Ok(file) => {
                let mut reader = csv::Reader::from_reader(file);
                for result in reader.records() {
                    let record = result.unwrap();
                    let timestamp_read = record.get(0).unwrap().parse::<DateTime<Utc>>().unwrap();
                    let connection_name = record.get(1).unwrap().to_string();
                    let up_rate = record.get(6).unwrap().parse::<u128>().unwrap();
                    let down_rate = record.get(7).unwrap().parse::<u128>().unwrap();
                    // Store the connection name and rate in the process_rate_data HashMap
                    self.connection_rate_data
                        .entry(connection_name)
                        .or_insert(BTreeMap::new())
                        .insert(timestamp_read,DataPoint {
                            value_up: up_rate,
                            value_down: down_rate,
                        });
                };
            } ,
            Err(_) => {},
        };
    }

    pub fn open_remote_address_rate_file(&mut self) {
        let file_path = "remote_addresses_record.csv";
        match File::open(file_path) {
            Ok(file) => {
                let mut reader = csv::Reader::from_reader(file);
                for result in reader.records() {
                    let record = result.unwrap();
                    let timestamp_read = record.get(0).unwrap().parse::<DateTime<Utc>>().unwrap();
                    let remote_address_name = record.get(1).unwrap().to_string();
                    let up_rate = record.get(2).unwrap().parse::<u128>().unwrap();
                    let down_rate = record.get(3).unwrap().parse::<u128>().unwrap();
                    // Store the process name and rate in the process_rate_data HashMap
                    self.remote_address_rate_data
                        .entry(remote_address_name)
                        .or_insert(BTreeMap::new())
                        .insert(timestamp_read,DataPoint {
                            value_up: up_rate,
                            value_down: down_rate,
                        });
                };
            } ,
            Err(_) => {},
        };
    }

    pub fn add_process_rate_data(&mut self, process_name: String, timestamp: i64, up_rate: u128, down_rate: u128) {
        let timestamp_read = DateTime::<Utc>::from_naive_utc_and_offset(NaiveDateTime::from_timestamp_opt(timestamp, 0).unwrap(), Utc);

        self.process_rate_data
            .entry(process_name)
            .or_insert(BTreeMap::new())
            .insert(timestamp_read,DataPoint {
                value_up: up_rate,
                value_down: down_rate,
            });
    }

    pub fn add_connection_rate_data(&mut self, connection_name: String, timestamp: i64, up_rate: u128, down_rate: u128) {
        
        let timestamp = DateTime::<Utc>::from_naive_utc_and_offset(NaiveDateTime::from_timestamp_opt(timestamp, 0).unwrap(), Utc);


        self.connection_rate_data
            .entry(connection_name)
            .or_insert(BTreeMap::new())
            .insert(timestamp,DataPoint {
                value_up: up_rate,
                value_down: down_rate,
            });
    }

    pub fn add_remote_address_rate_data(&mut self, remote_address_name: String, timestamp: i64, up_rate: u128, down_rate: u128) {
        
        let timestamp = DateTime::<Utc>::from_naive_utc_and_offset(NaiveDateTime::from_timestamp_opt(timestamp, 0).unwrap(), Utc);

        self.remote_address_rate_data
            .entry(remote_address_name)
            .or_insert(BTreeMap::new())
            .insert(timestamp,DataPoint {
                value_up: up_rate,
                value_down: down_rate,
            });
    }
}
