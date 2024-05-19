use std::{collections::HashMap, net::IpAddr, time::Duration};

use std::fs::OpenOptions;
use std::io::prelude::*;

use chrono::prelude::*;
use ratatui::{backend::Backend, Terminal};
use serde::Serialize;
use crate::{
    cli::{Opt, RenderOpts},
    display::{
        components::{HeaderDetails, HelpText, Layout, Table},
        UIState,DataCollector
    },
    network::{display_connection_string, display_ip_or_host, LocalSocket, Utilization},
    os::ProcessInfo,
};


#[derive(Serialize,Clone,Debug)]
pub struct FrontendTableData {
    pub title: String,
    pub column_names: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

pub struct Ui<B>
where
    B: Backend,
{
    terminal: Terminal<B>,
    state: UIState,
    ip_to_host: HashMap<IpAddr, String>,
    opts: RenderOpts,
    pub data_collector: DataCollector,
}

impl<B> Ui<B>
where
    B: Backend,
{
    pub fn new(terminal_backend: B, opts: &Opt) -> Self {
        let mut terminal = Terminal::new(terminal_backend).unwrap();
        terminal.clear().unwrap();
        terminal.hide_cursor().unwrap();
        let state = {
            let mut state = UIState::default();
            state.interface_name.clone_from(&opts.interface);
            state.unit_family = opts.render_opts.unit_family.into();
            state.cumulative_mode = opts.render_opts.total_utilization;
            state
        };

        let data_collector = {
            let mut data_collector = DataCollector::new();
            data_collector.open_files();
            data_collector
        };

        Ui {
            terminal,
            state,
            ip_to_host: Default::default(),
            opts: opts.render_opts,
            data_collector,
        }
    }
    pub fn check_alerts(&self, threshold: u64) {
        let threshold_bytes = threshold * 1024;
        for (proc_info, process_network_data) in &self.state.processes_total {
            let total_bandwidth = process_network_data.total_bytes_downloaded + process_network_data.total_bytes_uploaded;
            if total_bandwidth > threshold_bytes.into() {
                println!("Alert: Process with PID: {} has exceeded the bandwidth threshold of {} kilobytes",proc_info.pid, threshold);
            }
        }
    }
    //this function is used to output the process data to a file
    pub fn output_process_data_to_file(&mut self, file_path: &str) {
        let state = &self.state;
        let local_time: DateTime<Local> = Local::now();
        let timestamp = local_time.timestamp();
        let mut no_traffic = true;

        let mut output_process_data = |file: &mut std::fs::File, no_traffic: &mut bool| {
            let is_file_empty = file.metadata().map(|metadata| metadata.len() == 0).unwrap_or(false);
            if is_file_empty {
                writeln!(
                    file,
                    "timestamp,process_name,pid,bytes_uploaded,bytes_downloaded,connection_count"
                )
                .expect("Unable to write to file");
            }
            for (proc_info, process_network_data) in &state.processes {
                writeln!(
                    file,
                    "{},{},{},{},{},{}",
                    timestamp,
                    proc_info.name,
                    proc_info.pid,
                    process_network_data.total_bytes_uploaded,
                    process_network_data.total_bytes_downloaded,
                    process_network_data.connection_count
                )
                .expect("Unable to write to file");
                *no_traffic = false;
                self.data_collector.add_process_rate_data(proc_info.name.clone(), timestamp, process_network_data.total_bytes_uploaded, process_network_data.total_bytes_downloaded);
            }
        };

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(file_path)
            .expect("Unable to open file");

        output_process_data(&mut file, &mut no_traffic);
    }

    //this function is used to output the connection data to a file
    pub fn output_connections_data_to_file(&mut self, file_path: &str) {
        let state = &self.state;
        let ip_to_host = &self.ip_to_host;
        let local_time: DateTime<Local> = Local::now();
        let timestamp = local_time.timestamp();
        let mut no_traffic = true;

        let mut output_connections_data =
            |file: &mut std::fs::File, no_traffic: &mut bool| {
                let is_file_empty = file.metadata().map(|metadata| metadata.len() == 0).unwrap_or(false);
                if is_file_empty {
                    writeln!(
                        file,
                        "timestamp,interface_name,local_socket_port,target_ip,remote_socket_port,local_socket_protocol,bytes_uploaded,bytes_downloaded,process_name"
                    )
                    .expect("Unable to write to file");
                }
                for (connection, connection_network_data) in &state.connections {
                    writeln!(
                        file,
                        "{},{},{},{},{},{},{},{},{}",
                        timestamp,
                        &connection_network_data.interface_name,
                        connection.local_socket.port,
                        display_ip_or_host(connection.remote_socket.ip, ip_to_host),
                        connection.remote_socket.port,
                        connection.local_socket.protocol,
                        connection_network_data.total_bytes_uploaded,
                        connection_network_data.total_bytes_downloaded,
                        connection_network_data.process_name
                    ).expect("Unable to write to file");
                    *no_traffic = false;
                    
                    let connection_name = connection_network_data.interface_name.clone();

                    self.data_collector.add_connection_rate_data(connection_name,timestamp, connection_network_data.total_bytes_uploaded, connection_network_data.total_bytes_downloaded);
                }
            };

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(file_path)
            .expect("Unable to open file");

        output_connections_data(&mut file, &mut no_traffic);
    }

    //this function is used to output the remote address data to a file
    pub fn output_remote_addresses_data_to_file(&mut self, file_path: &str) {
        let state = &self.state;
        let ip_to_host = &self.ip_to_host;
        let local_time: DateTime<Local> = Local::now();
        let timestamp = local_time.timestamp();
        let mut no_traffic = true;

        let mut output_adressess_data = |file: &mut std::fs::File, no_traffic: &mut bool| {
            let is_file_empty = file.metadata().map(|metadata| metadata.len() == 0).unwrap_or(false);
            if is_file_empty {
                writeln!(
                    file,
                    "timestamp,remote_address,bytes_uploaded,bytes_downloaded,connections_count"
                )
                .expect("Unable to write to file");
            }
            for (remote_address, remote_address_network_data) in &state.remote_addresses {
                writeln!(
                    file,
                    "{},{},{},{},{}",
                    timestamp,
                    display_ip_or_host(*remote_address, ip_to_host),
                    remote_address_network_data.total_bytes_uploaded,
                    remote_address_network_data.total_bytes_downloaded,
                    remote_address_network_data.connection_count
                ).expect("Unable to write to file");
                *no_traffic = false;
                self.data_collector.add_remote_address_rate_data(display_ip_or_host(*remote_address, ip_to_host), timestamp, remote_address_network_data.total_bytes_uploaded, remote_address_network_data.total_bytes_downloaded);
            }
        };

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(file_path)
            .expect("Unable to open file");

        output_adressess_data(&mut file, &mut no_traffic);
    }
    
    //this function is used to output the total process data to a file
    pub fn output_process_total_data_to_file(&mut self, file_path: &str) {
        let state = &self.state;
        //let ip_to_host = &self.ip_to_host;
        let local_time: DateTime<Local> = Local::now();
        let timestamp = local_time.timestamp();
        let mut no_traffic = true;

        let mut output_process_data = |file: &mut std::fs::File, no_traffic: &mut bool| {
            let is_file_empty = file.metadata().map(|metadata| metadata.len() == 0).unwrap_or(false);
            if is_file_empty {
                writeln!(
                    file,
                    "timestamp,process_name,pid,bytes_uploaded,bytes_downloaded,connection_count"
                )
                .expect("Unable to write to file");
            }
            for (proc_info, process_network_data) in &state.processes_total {
                writeln!(
                    file,
                    "{},{},{},{},{},{}",
                    timestamp,
                    proc_info.name,
                    proc_info.pid,
                    process_network_data.total_bytes_uploaded,
                    process_network_data.total_bytes_downloaded,
                    process_network_data.connection_count
                )
                .expect("Unable to write to file");
                *no_traffic = false;
                let process_total_file_upload = self.data_collector.get_process_total_file_upload(proc_info.name.clone());
                let process_total_file_download = self.data_collector.get_process_total_file_download(proc_info.name.clone());
                self.data_collector.add_process_total_data(proc_info.name.clone(), timestamp,
                 process_network_data.total_bytes_uploaded + process_total_file_upload,
                  process_network_data.total_bytes_downloaded + process_total_file_download);
            }
        };

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(file_path)
            .expect("Unable to open file");

        output_process_data(&mut file, &mut no_traffic);
    }

    //this function is used to output the total connection data to a file
    pub fn output_connections_total_data_to_file(&mut self, file_path: &str) {
        let state = &self.state;
        let ip_to_host = &self.ip_to_host;
        let local_time: DateTime<Local> = Local::now();
        let timestamp = local_time.timestamp();
        let mut no_traffic = true;

        let mut output_connections_data =
            |file: &mut std::fs::File, no_traffic: &mut bool| {
                let is_file_empty = file.metadata().map(|metadata| metadata.len() == 0).unwrap_or(false);
                if is_file_empty {
                    writeln!(
                        file,
                        "timestamp,interface_name,local_socket_port,target_ip,remote_socket_port,local_socket_protocol,bytes_uploaded,bytes_downloaded,process_name"
                    )
                    .expect("Unable to write to file");
                }
                for (connection, connection_network_data) in &state.connections_total {
                    writeln!(
                        file,
                        "{},{},{},{},{},{},{},{},{}",
                        timestamp,
                        &connection_network_data.interface_name,
                        connection.local_socket.port,
                        display_ip_or_host(connection.remote_socket.ip, ip_to_host),
                        connection.remote_socket.port,
                        connection.local_socket.protocol,
                        connection_network_data.total_bytes_uploaded,
                        connection_network_data.total_bytes_downloaded,
                        connection_network_data.process_name
                    ).expect("Unable to write to file");
                    *no_traffic = false;

                    let connection_total_file_upload = self.data_collector.get_connection_total_file_upload(connection_network_data.interface_name.clone());
                    let connection_total_file_download = self.data_collector.get_connection_total_file_download(connection_network_data.interface_name.clone());
                    self.data_collector.add_connection_total_data(connection_network_data.interface_name.clone(),timestamp,
                     connection_network_data.total_bytes_uploaded + connection_total_file_upload,
                      connection_network_data.total_bytes_downloaded + connection_total_file_download);
                }
            };

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(file_path)
            .expect("Unable to open file");

        output_connections_data(&mut file, &mut no_traffic);
    }

    //this function is used to output the total remote address data to a file
    pub fn output_remote_addresses_total_data_to_file(&mut self, file_path: &str) {
        let state = &self.state;
        let ip_to_host = &self.ip_to_host;
        let local_time: DateTime<Local> = Local::now();
        let timestamp = local_time.timestamp();
        let mut no_traffic = true;

        let mut output_adressess_data = |file: &mut std::fs::File, no_traffic: &mut bool| {
            let is_file_empty = file.metadata().map(|metadata| metadata.len() == 0).unwrap_or(false);
            if is_file_empty {
                writeln!(
                    file,
                    "timestamp,remote_address,bytes_uploaded,bytes_downloaded,connections_count"
                )
                .expect("Unable to write to file");
            }
            for (remote_address, remote_address_network_data) in &state.remote_addresses_total {
                writeln!(
                    file,
                    "{},{},{},{},{}",
                    timestamp,
                    display_ip_or_host(*remote_address, ip_to_host),
                    remote_address_network_data.total_bytes_uploaded,
                    remote_address_network_data.total_bytes_downloaded,
                    remote_address_network_data.connection_count
                ).expect("Unable to write to file");
                *no_traffic = false;

                let remote_address_file_upload_total = self.data_collector.get_remote_address_total_file_upload(display_ip_or_host(*remote_address, ip_to_host));
                let remote_address_file_download_total = self.data_collector.get_remote_address_total_file_download(display_ip_or_host(*remote_address, ip_to_host));

                self.data_collector.add_remote_address_total_data(display_ip_or_host(*remote_address, ip_to_host), timestamp, 
                remote_address_network_data.total_bytes_uploaded + remote_address_file_upload_total,
                 remote_address_network_data.total_bytes_downloaded + remote_address_file_download_total);

            }
        };

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(file_path)
            .expect("Unable to open file");

        output_adressess_data(&mut file, &mut no_traffic);
    }



    pub fn output_text(&mut self, write_to_stdout: &mut (dyn FnMut(String) + Send)) {
        let state = &self.state;
        let ip_to_host = &self.ip_to_host;
        let local_time: DateTime<Local> = Local::now();
        let timestamp = local_time.timestamp();
        let mut no_traffic = true;

        let output_process_data = |write_to_stdout: &mut (dyn FnMut(String) + Send),
                                   no_traffic: &mut bool| {
            for (proc_info, process_network_data) in &state.processes {
                write_to_stdout(format!(
                    "process: <{timestamp}> \"{}\" up/down Bps: {}/{} connections: {}",
                    proc_info.name,
                    process_network_data.total_bytes_uploaded,
                    process_network_data.total_bytes_downloaded,
                    process_network_data.connection_count
                ));
                *no_traffic = false;
            }
        };

        let output_connections_data =
            |write_to_stdout: &mut (dyn FnMut(String) + Send), no_traffic: &mut bool| {
                for (connection, connection_network_data) in &state.connections {
                    write_to_stdout(format!(
                        "connection: <{timestamp}> {} up/down Bps: {}/{} process: \"{}\"",
                        display_connection_string(
                            connection,
                            ip_to_host,
                            &connection_network_data.interface_name,
                        ),
                        connection_network_data.total_bytes_uploaded,
                        connection_network_data.total_bytes_downloaded,
                        connection_network_data.process_name
                    ));
                    *no_traffic = false;
                }
            };

        let output_adressess_data = |write_to_stdout: &mut (dyn FnMut(String) + Send),
                                     no_traffic: &mut bool| {
            for (remote_address, remote_address_network_data) in &state.remote_addresses {
                write_to_stdout(format!(
                    "remote_address: <{timestamp}> {} up/down Bps: {}/{} connections: {}",
                    display_ip_or_host(*remote_address, ip_to_host),
                    remote_address_network_data.total_bytes_uploaded,
                    remote_address_network_data.total_bytes_downloaded,
                    remote_address_network_data.connection_count
                ));
                *no_traffic = false;
            }
        };

        // header
        write_to_stdout("Refreshing:".into());

        // body1
        if self.opts.processes {
            output_process_data(write_to_stdout, &mut no_traffic);
        }
        if self.opts.connections {
            output_connections_data(write_to_stdout, &mut no_traffic);
        }
        if self.opts.addresses {
            output_adressess_data(write_to_stdout, &mut no_traffic);
        }
        if !(self.opts.processes || self.opts.connections || self.opts.addresses) {
            output_process_data(write_to_stdout, &mut no_traffic);
            output_connections_data(write_to_stdout, &mut no_traffic);
            output_adressess_data(write_to_stdout, &mut no_traffic);
        }

        // body2: In case no traffic is detected
        if no_traffic {
            write_to_stdout("<NO TRAFFIC>".into());
        }

        // footer
        write_to_stdout("".into());
    }

    pub fn draw(&mut self, paused: bool, show_dns: bool, elapsed_time: Duration, ui_offset: usize) {
        let layout = Layout {
            header: HeaderDetails {
                state: &self.state,
                elapsed_time,
                paused,
            },
            children: self.get_tables_to_display(),
            footer: HelpText { paused, show_dns },
        };
        self.terminal
            .draw(|frame| layout.render(frame, frame.size(), ui_offset))
            .unwrap();
    }

    fn get_tables_to_display(&self) -> Vec<Table> {
        let opts = &self.opts;
        let mut children: Vec<Table> = Vec::new();
        if opts.processes {
            children.push(Table::create_processes_table(&self.state));
        }
        if opts.addresses {
            children.push(Table::create_remote_addresses_table(
                &self.state,
                &self.ip_to_host,
            ));
        }
        if opts.connections {
            children.push(Table::create_connections_table(
                &self.state,
                &self.ip_to_host,
            ));
        }
        if !(opts.processes || opts.addresses || opts.connections) {
            children = vec![
                Table::create_processes_table(&self.state),
                Table::create_remote_addresses_table(&self.state, &self.ip_to_host),
                Table::create_connections_table(&self.state, &self.ip_to_host),
            ];
        }
        children
    }

    pub fn get_table_count(&self) -> usize {
        self.get_tables_to_display().len()
    }

    pub fn update_state(
        &mut self,
        connections_to_procs: HashMap<LocalSocket, ProcessInfo>,
        utilization: Utilization,
        ip_to_host: HashMap<IpAddr, String>,
    ) {
        self.state.update(connections_to_procs, utilization);
        self.ip_to_host.extend(ip_to_host);
    }
    pub fn end(&mut self) {
        self.terminal.show_cursor().unwrap();
    }
    pub fn get_draw_data(&self) -> Vec<FrontendTableData> {
        let tables = self.get_tables_to_display();
        let mut frontend_data = Vec::new();

        for table in tables {
            let column_names = table.get_data().column_names().iter().map(|&s| s.to_string()).collect();
            let rows = table.get_data().rows().into_iter().map(|r| r.to_vec()).collect();

            let data = FrontendTableData {
                title: table.get_title().to_string(),
                column_names,
                rows,
            };

            frontend_data.push(data);
        }

        frontend_data
    }
}
