
use std::{collections::{BTreeMap, HashMap}, fs::File, io::BufReader};

use csv::{ReaderBuilder, StringRecord};
use std::path::Path;

use chrono::prelude::*;

pub struct DataPoint {
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
            remote_address_total_data_from_file: HashMap::new()
        }
    }

    pub fn open_files(&mut self) {
        self.open_process_rate_file();
        self.open_connection_rate_file();
        self.open_remote_address_rate_file();
        self.open_process_total_file();
        self.open_connection_total_file();
        self.open_remote_address_total_file();
    }

    pub fn open_process_rate_file(&mut self) {
        let file_path = "process_record.csv";
        if Path::new(file_path).exists() {
            
        
            let file = File::open(file_path).expect("Oh no");
            
            let mut reader = ReaderBuilder::new()
            .has_headers(true)
            .from_reader(BufReader::new(file));
            for result in reader.records() {
                
                let record: StringRecord = result.expect("No, just no");
                let temp_timestamp = record[0].trim().parse::<i64>().unwrap();
                let timestamp_read = DateTime::<Utc>::from_naive_utc_and_offset(NaiveDateTime::from_timestamp_opt(temp_timestamp, 0).unwrap(), Utc);

                
                let process_name = record[1].to_string();
                let up_rate = record[2].parse::<u128>().unwrap();
                let down_rate = record[3].parse::<u128>().unwrap();
                // Store the process name and rate in the process_rate_data HashMap
                self.process_rate_data
                    .entry(process_name)
                    .or_insert(BTreeMap::new())
                    .insert(timestamp_read, DataPoint {
                        value_up: up_rate,
                        value_down: down_rate,
                    });
                
            }
        }
    }

    pub fn open_connection_rate_file(&mut self) {
        let file_path = "connection_record.csv";
        if Path::new(file_path).exists() {
            let file = File::open(file_path).expect("Oh no");
            
            let mut reader = ReaderBuilder::new()
            .has_headers(true)
            .from_reader(BufReader::new(file));
            for result in reader.records() {
                
                let record: StringRecord = result.expect("No, just no");
                let temp_timestamp = record[0].trim().parse::<i64>().unwrap();
                let timestamp_read = DateTime::<Utc>::from_naive_utc_and_offset(NaiveDateTime::from_timestamp_opt(temp_timestamp, 0).unwrap(), Utc);

                
                let connection_name = record[1].to_string();
                let up_rate = record[6].parse::<u128>().unwrap();
                let down_rate = record[7].parse::<u128>().unwrap();
                // Store the process name and rate in the process_rate_data HashMap
                self.connection_rate_data
                    .entry(connection_name)
                    .or_insert(BTreeMap::new())
                    .insert(timestamp_read, DataPoint {
                        value_up: up_rate,
                        value_down: down_rate,
                    });
                
            }
        }
    }

    pub fn open_remote_address_rate_file(&mut self) {
        let file_path = "remote_addresses_record.csv";
        if Path::new(file_path).exists(){
            let file = File::open(file_path).expect("Oh no");
            
            let mut reader = ReaderBuilder::new()
            .has_headers(true)
            .from_reader(BufReader::new(file));
            for result in reader.records() {
                
                let record: StringRecord = result.expect("No, just no");
                let temp_timestamp = record[0].trim().parse::<i64>().unwrap();
                let timestamp_read = DateTime::<Utc>::from_naive_utc_and_offset(NaiveDateTime::from_timestamp_opt(temp_timestamp, 0).unwrap(), Utc);

                
                let remote_address_name = record[1].to_string();
                let up_rate = record[2].parse::<u128>().unwrap();
                let down_rate = record[3].parse::<u128>().unwrap();
                // Store the process name and rate in the process_rate_data HashMap
                self.remote_address_rate_data
                    .entry(remote_address_name)
                    .or_insert(BTreeMap::new())
                    .insert(timestamp_read, DataPoint {
                        value_up: up_rate,
                        value_down: down_rate,
                    });
                
            }
        }
    }

    

    pub fn open_process_total_file(&mut self) {
        let file_path = "process_total_record.csv";

        let mut last_value_up = 0;
        let mut last_value_down = 0;
        let mut last_run_total_up = 0;
        let mut last_run_total_down = 0;

        if Path::new(file_path).exists(){

            let file = File::open(file_path).expect("Oh no");
            
            let mut reader = ReaderBuilder::new()
            .has_headers(true)
            .from_reader(BufReader::new(file));
            for result in reader.records() {
                
                let record: StringRecord = result.expect("No, just no");
                let temp_timestamp = record[0].trim().parse::<i64>().unwrap();
                let timestamp_read = DateTime::<Utc>::from_naive_utc_and_offset(NaiveDateTime::from_timestamp_opt(temp_timestamp, 0).unwrap(), Utc);

                
                let process_name = record[1].to_string();
                let up_total = record[2].parse::<u128>().unwrap();
                let down_total = record[3].parse::<u128>().unwrap();
                // Store the process name and rate in the process_rate_data HashMap
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
                
            }
        }
        
        
    }

    pub fn open_connection_total_file(&mut self) {
        let file_path = "connection_total_record.csv";

        let mut last_value_up = 0;
        let mut last_value_down = 0;
        let mut last_run_total_up = 0;
        let mut last_run_total_down = 0;

        if Path::new(file_path).exists() {
            let file = File::open(file_path).expect("Oh no");
            
            let mut reader = ReaderBuilder::new()
            .has_headers(true)
            .from_reader(BufReader::new(file));
            for result in reader.records() {
                
                let record: StringRecord = result.expect("No, just no");
                let temp_timestamp = record[0].trim().parse::<i64>().unwrap();
                let timestamp_read = DateTime::<Utc>::from_naive_utc_and_offset(NaiveDateTime::from_timestamp_opt(temp_timestamp, 0).unwrap(), Utc);

                
                let connection_name = record[1].to_string();
                let up_total = record[6].parse::<u128>().unwrap();
                let down_total = record[7].parse::<u128>().unwrap();
                // Store the process name and rate in the process_rate_data HashMap
                if (last_value_up >= up_total) || (last_value_down >= down_total) {
                    last_run_total_up = last_value_up;
                    last_run_total_down = last_value_down;
                }
                
                // Store the process name and rate in the process_rate_data HashMap
                let connection_name2 = connection_name.clone();
                        // Store the process name and rate in the process_rate_data HashMap
                self.connection_total_data
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
            }
        }
    }

    pub fn open_remote_address_total_file(&mut self) {
        let file_path = "remote_addresses_total_record.csv";

        let mut last_value_up = 0;
        let mut last_value_down = 0;
        let mut last_run_total_up = 0;
        let mut last_run_total_down = 0;

        if Path::new(file_path).exists() {

            let file = File::open(file_path).expect("Oh no");
            
            let mut reader = ReaderBuilder::new()
            .has_headers(true)
            .from_reader(BufReader::new(file));
            for result in reader.records() {
                
                let record: StringRecord = result.expect("No, just no");
                let temp_timestamp = record[0].trim().parse::<i64>().unwrap();
                let timestamp_read = DateTime::<Utc>::from_naive_utc_and_offset(NaiveDateTime::from_timestamp_opt(temp_timestamp, 0).unwrap(), Utc);

                
                let remote_address_name = record[1].to_string();
                let up_total = record[2].parse::<u128>().unwrap();
                let down_total = record[3].parse::<u128>().unwrap();
                // Store the process name and rate in the process_rate_data HashMap
                if (last_value_up >= up_total) || (last_value_down >= down_total) {
                    last_run_total_up = last_value_up;
                    last_run_total_down = last_value_down;
                }

                let remote_address_name2 = remote_address_name.clone();
                
                // Store the process name and rate in the process_rate_data HashMap
                self.remote_address_total_data
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
                
            }
        }
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

    pub fn get_process_rate_subset(&self, process_name: &str, start_datetime: DateTime<Utc>) -> BTreeMap<DateTime<Utc>, &DataPoint> {
        let mut subset = BTreeMap::new();
        match self.process_rate_data.get(process_name) {
            Some(data) => {
                for (timestamp, value) in data {
                    if timestamp >= &start_datetime {
                        subset.insert(timestamp.clone(), value);
                    }
                }
            },
            None => {},
        }
        subset
    }

    pub fn get_connection_rate_subset(&self, connection_name: &str, start_datetime: DateTime<Utc>) -> BTreeMap<DateTime<Utc>, &DataPoint> {
        let mut subset = BTreeMap::new();
        match self.connection_rate_data.get(connection_name) {
            Some(data) => {
                for (timestamp, value) in data {
                    if timestamp >= &start_datetime {
                        subset.insert(timestamp.clone(), value);
                    }
                }
            },
            None => {},
        }
        subset
    }

    pub fn get_remote_address_rate_subset(&self, remote_address_name: &str, start_datetime: DateTime<Utc>) -> BTreeMap<DateTime<Utc>, &DataPoint> {
        let mut subset = BTreeMap::new();
        match self.remote_address_rate_data.get(remote_address_name) {
            Some(data) => {
                for (timestamp, value) in data {
                    if timestamp >= &start_datetime {
                        subset.insert(timestamp.clone(), value);
                    }
                }
            },
            None => {},
        }
        subset
    }

    pub fn get_process_total_subset(&self, process_name: &str, start_datetime: DateTime<Utc>) -> BTreeMap<DateTime<Utc>, &DataPoint> {
        let mut subset = BTreeMap::new();
        match self.process_total_data.get(process_name) {
            Some(data) => {
                for (timestamp, value) in data {
                    if timestamp >= &start_datetime {
                        subset.insert(timestamp.clone(), value);
                    }
                }
            },
            None => {},
        }
        subset
    }

    pub fn get_connection_total_subset(&self, connection_name: &str, start_datetime: DateTime<Utc>) -> BTreeMap<DateTime<Utc>, &DataPoint> {
        let mut subset = BTreeMap::new();
        match self.connection_total_data.get(connection_name) {
            Some(data) => {
                for (timestamp, value) in data {
                    if timestamp >= &start_datetime {
                        subset.insert(timestamp.clone(), value);
                    }
                }
            },
            None => {},
        }
        subset
    }

    pub fn get_remote_address_total_subset(&self, remote_address_name: &str, start_datetime: DateTime<Utc>) -> BTreeMap<DateTime<Utc>, &DataPoint> {
        let mut subset = BTreeMap::new();
        match self.remote_address_total_data.get(remote_address_name) {
            Some(data) => {
                for (timestamp, value) in data {
                    if timestamp >= &start_datetime {
                        subset.insert(timestamp.clone(), value);
                    }
                }
            },
            None => {},
        }
        subset
    }

    pub fn save_process_rate_data(&self, start_datetime: DateTime<Utc>) {
        let file_path = "process_data_collected.csv";
        let mut writer = csv::Writer::from_path(file_path).unwrap();
        
        for (process_name, _) in &self.process_rate_data {
            let subset = self.get_process_rate_subset(process_name, start_datetime);
            for (timestamp, value) in subset {
                writer.write_record(&[
                    process_name.clone(),
                    timestamp.to_rfc3339(),
                    value.value_up.to_string(),
                    value.value_down.to_string(),
                ]).unwrap();
            }
        }
        writer.flush().unwrap();
    }

    pub fn save_connection_rate_data(&self, start_datetime: DateTime<Utc>) {
        let file_path = "connection_data_collected.csv";
        let mut writer = csv::Writer::from_path(file_path).unwrap();
        for (connection_name,_) in &self.connection_rate_data {
            let subset = self.get_connection_rate_subset(connection_name, start_datetime);
            for (timestamp, value) in subset {
                writer.write_record(&[
                    connection_name.clone(),
                    timestamp.to_rfc3339(),
                    value.value_up.to_string(),
                    value.value_down.to_string(),
                ]).unwrap();
            }
        }
        writer.flush().unwrap();
    }

    pub fn save_remote_address_rate_data(&self, start_datetime: DateTime<Utc>) {
        let file_path = "remote_address_data_collected.csv";
        let mut writer = csv::Writer::from_path(file_path).unwrap();
        for (remote_address_name,_) in &self.remote_address_rate_data {
            let subset = self.get_remote_address_rate_subset(remote_address_name, start_datetime);
            for (timestamp, value) in subset {
                writer.write_record(&[
                    remote_address_name.clone(),
                    timestamp.to_rfc3339(),
                    value.value_up.to_string(),
                    value.value_down.to_string(),
                ]).unwrap();
            }
        }
        writer.flush().unwrap();
    }

    pub fn save_process_total_data(&self, start_datetime: DateTime<Utc>) {
        let file_path = "process_total_data_collected.csv";
        let mut writer = csv::Writer::from_path(file_path).unwrap();
        for (process_name,_) in &self.process_total_data {
            let subset = self.get_process_total_subset(process_name, start_datetime);
            for (timestamp, value) in subset {
                writer.write_record(&[
                    process_name.clone(),
                    timestamp.to_rfc3339(),
                    value.value_up.to_string(),
                    value.value_down.to_string(),
                ]).unwrap();
            }
        }
        writer.flush().unwrap();
    }

    pub fn save_connection_total_data(&self, start_datetime: DateTime<Utc>) {
        let file_path = "connection_total_data_collected.csv";
        let mut writer = csv::Writer::from_path(file_path).unwrap();
        for (connection_name,_) in &self.connection_total_data {
            let subset = self.get_connection_total_subset(connection_name, start_datetime);
            for (timestamp, value) in subset {
                writer.write_record(&[
                    connection_name.clone(),
                    timestamp.to_rfc3339(),
                    value.value_up.to_string(),
                    value.value_down.to_string(),
                ]).unwrap();
            }
        }
        writer.flush().unwrap();
    }

    pub fn save_remote_address_total_data(&self, start_datetime: DateTime<Utc>) {
        let file_path = "remote_address_total_data_collected.csv";
        let mut writer = csv::Writer::from_path(file_path).unwrap();
        for (remote_address_name,_) in &self.remote_address_total_data {
            let subset = self.get_remote_address_total_subset(remote_address_name, start_datetime);
            for (timestamp, value) in subset {
                writer.write_record(&[
                    remote_address_name.clone(),
                    timestamp.to_rfc3339(),
                    value.value_up.to_string(),
                    value.value_down.to_string(),
                ]).unwrap();
            }
        }
        writer.flush().unwrap();
    }



}
