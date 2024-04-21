use std::{collections::HashMap, fs::File};

use std::fs::OpenOptions;

use chrono::prelude::*;

struct DataPoint {
    timestamp: DateTime<Utc>,
    value_up: f64,
    value_down: f64,
}

pub struct DataCollector where{
    process_rate_data: HashMap<String, Vec<DataPoint>>,
    connection_rate_data: HashMap<String, Vec<DataPoint>>,
    remote_address_rate_data: HashMap<String, Vec<DataPoint>>,
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
                    let up_rate = record.get(2).unwrap().parse::<f64>().unwrap();
                    let down_rate = record.get(3).unwrap().parse::<f64>().unwrap();
                    // Store the process name and rate in the process_rate_data HashMap
                    self.process_rate_data
                        .entry(process_name)
                        .or_insert(Vec::new())
                        .push(DataPoint {
                            timestamp: timestamp_read,
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
                    let up_rate = record.get(6).unwrap().parse::<f64>().unwrap();
                    let down_rate = record.get(7).unwrap().parse::<f64>().unwrap();
                    // Store the connection name and rate in the process_rate_data HashMap
                    self.connection_rate_data
                        .entry(connection_name)
                        .or_insert(Vec::new())
                        .push(DataPoint {
                            timestamp: timestamp_read,
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
                    let up_rate = record.get(2).unwrap().parse::<f64>().unwrap();
                    let down_rate = record.get(3).unwrap().parse::<f64>().unwrap();
                    // Store the process name and rate in the process_rate_data HashMap
                    self.remote_address_rate_data
                        .entry(remote_address_name)
                        .or_insert(Vec::new())
                        .push(DataPoint {
                            timestamp: timestamp_read,
                            value_up: up_rate,
                            value_down: down_rate,
                        });
                };
            } ,
            Err(_) => {},
        };
    }
}
