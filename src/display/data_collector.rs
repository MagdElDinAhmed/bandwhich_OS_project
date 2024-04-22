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
    process_total_data: HashMap<String, BTreeMap<DateTime<Utc>,DataPoint>>,
    connection_total_data: HashMap<String, BTreeMap<DateTime<Utc>,DataPoint>>,
    remote_address_total_data: HashMap<String, BTreeMap<DateTime<Utc>,DataPoint>>,
    process_total_data_from_file: HashMap<String, DataPoint>,
    connection_total_data_from_file: HashMap<String, DataPoint>,
    remote_address_total_data_from_file: HashMap<String, DataPoint>,
}

impl DataCollector {
    pub fn new() -> Self {
        Self {
            process_rate_data: HashMap::new(),
            connection_rate_data: HashMap::new(),
            remote_address_rate_data: HashMap::new(),
            process_total_data: HashMap::new(),
            connection_total_data: HashMap::new(),
            remote_address_total_data: HashMap::new(),
            process_total_data_from_file: HashMap::new(),
            connection_total_data_from_file: HashMap::new(),
            remote_address_total_data_from_file: HashMap::new(),
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

    pub fn open_process_total_file(&mut self) {
        let file_path = "process_total_record.csv";

        let mut last_value_up = 0;
        let mut last_value_down = 0;
        let mut last_run_total_up = 0;
        let mut last_run_total_down = 0;
        match File::open(file_path) {
            Ok(file) => {
                let mut reader = csv::Reader::from_reader(file);
                for result in reader.records() {
                    let record = result.unwrap();
                    let timestamp_read = record.get(0).unwrap().parse::<DateTime<Utc>>().unwrap();
                    let process_name = record.get(1).unwrap().to_string();
                    let up_total = record.get(2).unwrap().parse::<u128>().unwrap();
                    let down_total = record.get(3).unwrap().parse::<u128>().unwrap();
                    
                    if (last_value_up >= up_total) || (last_value_down >= down_total) {
                        last_run_total_up = last_value_up;
                        last_run_total_down = last_value_down;
                    }

                    let process_name2 = process_name.clone();
                    
                    // Store the process name and rate in the process_rate_data HashMap
                    self.process_total_data
                        .entry(process_name)
                        .or_insert(BTreeMap::new())
                        .insert(timestamp_read,DataPoint {
                            value_up: up_total + last_run_total_up,
                            value_down: down_total + last_run_total_down,
                        });
                    
                    last_value_up = up_total;
                    last_value_down = down_total;
                    self.process_total_data_from_file.insert(process_name2, DataPoint {
                        value_up: up_total,
                        value_down: down_total,
                    });
                };
            } ,
            Err(_) => {},
        };
        
    }

    pub fn open_connection_total_file(&mut self) {
        let file_path = "connection_total_record.csv";

        let mut last_value_up = 0;
        let mut last_value_down = 0;
        let mut last_run_total_up = 0;
        let mut last_run_total_down = 0;
        match File::open(file_path) {
            Ok(file) => {
                let mut reader = csv::Reader::from_reader(file);
                for result in reader.records() {
                    let record = result.unwrap();
                    let timestamp_read = record.get(0).unwrap().parse::<DateTime<Utc>>().unwrap();
                    let connection_name = record.get(1).unwrap().to_string();
                    let up_total = record.get(6).unwrap().parse::<u128>().unwrap();
                    let down_total = record.get(7).unwrap().parse::<u128>().unwrap();
                    
                    if (last_value_up >= up_total) || (last_value_down >= down_total) {
                        last_run_total_up = last_value_up;
                        last_run_total_down = last_value_down;
                    }
                    
                    let connection_name2 = connection_name.clone();
                    // Store the process name and rate in the process_rate_data HashMap
                    self.connection_rate_data
                        .entry(connection_name)
                        .or_insert(BTreeMap::new())
                        .insert(timestamp_read,DataPoint {
                            value_up: up_total + last_run_total_up,
                            value_down: down_total + last_run_total_down,
                        });
                    
                    last_value_up = up_total;
                    last_value_down = down_total;

                    self.connection_total_data_from_file.insert(connection_name2, DataPoint {
                        value_up: up_total,
                        value_down: down_total,
                    });
                };
            } ,
            Err(_) => {},
        };
    }

    pub fn open_remote_address_total_file(&mut self) {
        let file_path = "remote_addresses_total_record.csv";

        let mut last_value_up = 0;
        let mut last_value_down = 0;
        let mut last_run_total_up = 0;
        let mut last_run_total_down = 0;
        match File::open(file_path) {
            Ok(file) => {
                let mut reader = csv::Reader::from_reader(file);
                for result in reader.records() {
                    let record = result.unwrap();
                    let timestamp_read = record.get(0).unwrap().parse::<DateTime<Utc>>().unwrap();
                    let remote_address_name = record.get(1).unwrap().to_string();
                    let up_total = record.get(2).unwrap().parse::<u128>().unwrap();
                    let down_total = record.get(3).unwrap().parse::<u128>().unwrap();
                    
                    if (last_value_up >= up_total) || (last_value_down >= down_total) {
                        last_run_total_up = last_value_up;
                        last_run_total_down = last_value_down;
                    }
                    
                    let remote_address_name2 = remote_address_name.clone();
                    // Store the process name and rate in the process_rate_data HashMap
                    self.remote_address_rate_data
                        .entry(remote_address_name)
                        .or_insert(BTreeMap::new())
                        .insert(timestamp_read,DataPoint {
                            value_up: up_total + last_run_total_up,
                            value_down: down_total + last_run_total_down,
                        });
                    
                    last_value_up = up_total;
                    last_value_down = down_total;

                    self.remote_address_total_data_from_file.insert(remote_address_name2, DataPoint {
                        value_up: up_total,
                        value_down: down_total,
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

    pub fn add_process_total_data(&mut self, process_name: String, timestamp: i64, up_total: u128, down_total: u128) {
        
        let timestamp = DateTime::<Utc>::from_naive_utc_and_offset(NaiveDateTime::from_timestamp_opt(timestamp, 0).unwrap(), Utc);

        self.process_total_data
            .entry(process_name)
            .or_insert(BTreeMap::new())
            .insert(timestamp,DataPoint {
                value_up: up_total,
                value_down: down_total,
            });
    }

    pub fn add_connection_total_data(&mut self, connection_name: String, timestamp: i64, up_total: u128, down_total: u128) {
        
        let timestamp = DateTime::<Utc>::from_naive_utc_and_offset(NaiveDateTime::from_timestamp_opt(timestamp, 0).unwrap(), Utc);

        self.connection_total_data
            .entry(connection_name)
            .or_insert(BTreeMap::new())
            .insert(timestamp,DataPoint {
                value_up: up_total,
                value_down: down_total,
            });
    }

    pub fn add_remote_address_total_data(&mut self, remote_address_name: String, timestamp: i64, up_total: u128, down_total: u128) {
        
        let timestamp = DateTime::<Utc>::from_naive_utc_and_offset(NaiveDateTime::from_timestamp_opt(timestamp, 0).unwrap(), Utc);

        self.remote_address_total_data
            .entry(remote_address_name)
            .or_insert(BTreeMap::new())
            .insert(timestamp,DataPoint {
                value_up: up_total,
                value_down: down_total,
            });
    }

    pub fn get_process_total_file_upload(&self, process_name: String) -> u128 {
        match self.process_total_data_from_file.get(&process_name) {
            Some(data) => {
                data.value_up
            },
            None => 0,
        }
    }

    pub fn get_process_total_file_download(&self, process_name: String) -> u128 {
        match self.process_total_data_from_file.get(&process_name) {
            Some(data) => {
                data.value_down
            },
            None => 0,
        }
    }

    pub fn get_connection_total_file_upload(&self, connection_name: String) -> u128 {
        match self.connection_total_data_from_file.get(&connection_name) {
            Some(data) => {
                data.value_up
            },
            None => 0,
        }
    }

    pub fn get_connection_total_file_download(&self, connection_name: String) -> u128 {
        match self.connection_total_data_from_file.get(&connection_name) {
            Some(data) => {
                data.value_down
            },
            None => 0,
        }
    }

    pub fn get_remote_address_total_file_upload(&self, remote_address_name: String) -> u128 {
        match self.remote_address_total_data_from_file.get(&remote_address_name) {
            Some(data) => {
                data.value_up
            },
            None => 0,
        }
    }

    pub fn get_remote_address_total_file_download(&self, remote_address_name: String) -> u128 {
        match self.remote_address_total_data_from_file.get(&remote_address_name) {
            Some(data) => {
                data.value_down
            },
            None => 0,
        }
    }

}
