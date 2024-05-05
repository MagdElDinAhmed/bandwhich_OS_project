#![deny(clippy::enum_glob_use)]

mod cli;
mod display;
mod network;
mod os;
#[cfg(test)]
mod tests;

use std::{
    collections::{BTreeMap,HashMap},
    fs::File,
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering}, Arc, Mutex, MutexGuard, RwLock
    },
    thread::{self, park_timeout},
    time::{Duration, Instant}, vec,

};

use chrono::{Utc, prelude::*};
use clap::Parser;
use crossterm::{
    event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    terminal,
};
use display::{elapsed_time, RawTerminalBackend, Ui, DataPoint};
use network::{
    dns::{self, IpTable},
    LocalSocket, Sniffer, Utilization,
};
use pnet::datalink::{DataLinkReceiver, NetworkInterface};
use ratatui::backend::{Backend, CrosstermBackend};
use simplelog::WriteLogger;

use crate::cli::Opt;
use crate::os::ProcessInfo;
use lazy_static::lazy_static;
use tauri::{Manager, State};

//const DISPLAY_DELTA: Duration = Duration::from_millis(1000);

// use std::process::Command;
// use std::net::ToSocketAddrs;

use network::throttling::{set_egress_bandwidth_limit, set_ingress_bandwidth_limit};


// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn gpl() -> Vec<String> {
    let mut v_temp = PROCESS_LIST.lock().unwrap();
    let mut v = get_list(&v_temp);
    v
}

#[tauri::command]
fn gcl() -> Vec<String> {
    let mut v_temp = CONNECTION_LIST.lock().unwrap();
    let mut v = get_list(&v_temp);
    v
}

#[tauri::command]
fn gral() -> Vec<String> {
    let mut v_temp = REMOTE_ADDRESS_LIST.lock().unwrap();
    let mut v = get_list(&v_temp);
    v
}

#[tauri::command]
fn gpr(process: &str, time: &str) -> Vec<Vec<String>> {
    let mut v_temp = PROCESS_RATES.lock().unwrap();
    let mut v = get_rates_and_totals(process, time, &v_temp);
    return v;

}

#[tauri::command]
fn gcr(connection: &str, time: &str) -> Vec<Vec<String>> {
    let mut v_temp = CONNECTION_RATES.lock().unwrap();
    let mut v = get_rates_and_totals(connection, time, &v_temp);
    return v;

}

#[tauri::command]
fn grar(remote_address: &str, time: &str) -> Vec<Vec<String>> {
    let mut v_temp = REMOTE_ADDRESS_RATES.lock().unwrap();
    let mut v = get_rates_and_totals(remote_address, time, &v_temp);
    return v;
}

#[tauri::command]
fn gpt(process: &str, time: &str) -> Vec<Vec<String>> {
    let mut v_temp = PROCESS_TOTALS.lock().unwrap();
    let mut v = get_rates_and_totals(process, time, &v_temp);
    return v;
}

#[tauri::command]
fn gct(connection: &str, time: &str) -> Vec<Vec<String>> {
    let mut v_temp = CONNECTION_TOTALS.lock().unwrap();
    let mut v = get_rates_and_totals(connection, time, &v_temp);
    return v;
}

#[tauri::command]
fn grat(remote_address: &str, time: &str) -> Vec<Vec<String>> {
    let mut v_temp = REMOTE_ADDRESS_TOTALS.lock().unwrap();
    let mut v = get_rates_and_totals(remote_address, time, &v_temp);
    return v;

}

fn get_list(v_temp: &MutexGuard<Vec<String>>) -> Vec<String> {
    let mut v = Vec::new();
    for i in v_temp.iter() {
        v.push(i.clone());
    }
    v
}

fn get_rates_and_totals(name: &str,time: &str, v_temp: &MutexGuard<HashMap<String, BTreeMap<DateTime<Utc>,DataPoint>>>) -> Vec<Vec<String>>{
    let mut v = Vec::new();
    let mut start_time = Utc::now() - Duration::from_secs(60 * 60 * 24 * 30);

    if (time == "Last Year")
    {
        start_time = Utc::now() - Duration::from_secs(60 * 60 * 24 * 365);
    }
    else if (time == "Last Week")
    {
        start_time = Utc::now() - Duration::from_secs(60 * 60 * 24 * 7);
    }
    else {
        start_time = Utc.timestamp_opt(0, 0).unwrap();
    }
    

    let mut subset = BTreeMap::new();
    match v_temp.get(name) {
        Some(data) => {
            for (timestamp, value) in data {
                if timestamp >= &start_time {
                    subset.insert(timestamp.clone(), value);
                }
            }
        },
        None => {},
    }
    for (timestamp, value) in subset {
        let time = timestamp.to_rfc3339();
        let upload = value.value_up.to_string();
        let download = value.value_down.to_string();
        v.push(vec![time, upload, download]);
    }
    if (v.len() == 0) {
        v.push(vec!["No data available".to_string(), "No data available".to_string(), "No data available".to_string()]);
    }
    v
}


lazy_static! {
    static ref PROCESS_LIST: Mutex<Vec<String>> = Mutex::new(Vec::new());
    static ref CONNECTION_LIST: Mutex<Vec<String>> = Mutex::new(Vec::new());
    static ref REMOTE_ADDRESS_LIST: Mutex<Vec<String>> = Mutex::new(Vec::new());

    static ref PROCESS_RATES: Mutex<HashMap<String, BTreeMap<DateTime<Utc>,DataPoint>>> = Mutex::new(HashMap::new());
    static ref CONNECTION_RATES: Mutex<HashMap<String, BTreeMap<DateTime<Utc>,DataPoint>>> = Mutex::new(HashMap::new());
    static ref REMOTE_ADDRESS_RATES: Mutex<HashMap<String, BTreeMap<DateTime<Utc>,DataPoint>>> = Mutex::new(HashMap::new());

    static ref PROCESS_TOTALS: Mutex<HashMap<String, BTreeMap<DateTime<Utc>,DataPoint>>> = Mutex::new(HashMap::new());
    static ref CONNECTION_TOTALS: Mutex<HashMap<String, BTreeMap<DateTime<Utc>,DataPoint>>> = Mutex::new(HashMap::new());
    static ref REMOTE_ADDRESS_TOTALS: Mutex<HashMap<String, BTreeMap<DateTime<Utc>,DataPoint>>> = Mutex::new(HashMap::new());

}
fn main() -> anyhow::Result<()> {
    
    
    

    println!("This should print immediately when the program runs.");


    

    // let upload_limit_mbps = 10;
    // let download_limit_mbps = 1;
    let opts = Opt::parse();
    // Retrieve all network interfaces
    for iface in pnet::datalink::interfaces() {
        if !iface.is_up() || iface.ips.is_empty() {
            continue; // Skip interfaces that are down or have no IP
        }

        println!("Applying limits to interface: {}", iface.name);
        match set_egress_bandwidth_limit(&iface.name, opts.upload_limit_mbps) {
            Ok(_) => println!("Upload limit set successfully for {}", iface.name),
            Err(e) => eprintln!("Failed to set upload limit for {}: {}", iface.name, e),
        }

        match set_ingress_bandwidth_limit(&iface.name, opts.download_limit_mbps) {
            Ok(_) => println!("Download limit set successfully for {} with limit speed {}", iface.name, opts.download_limit_mbps),
            Err(e) => eprintln!("Failed to set download limit for {}: {}", iface.name, e),
        }
    }

    


    let opts = Opt::parse();

    // init logging
    if let Some(ref log_path) = opts.log_to {
        let log_file = File::options()
            .write(true)
            .create_new(true)
            .open(log_path)?;
        WriteLogger::init(
            opts.verbosity.log_level_filter(),
            Default::default(),
            log_file,
        )?;
    }

    let os_input = os::get_input(opts.interface.as_deref(), !opts.no_resolve, opts.dns_server)?;
    if opts.raw {
        let terminal_backend = RawTerminalBackend {};
        //let terminal_backend_clone = RawTerminalBackend {};
        //let ui = Arc::new(Mutex::new(Ui::new(terminal_backend_clone, &opts)));
          
        let backend = thread::spawn(move || {
            start(terminal_backend, os_input, opts);
        });
        
        tauri::Builder::default()
            .invoke_handler(tauri::generate_handler![greet, gpl, gcl, gral, gpr, gcr, grar, gpt, gct, grat])
            .run(tauri::generate_context!())
            .expect("error while running tauri application");
        
        backend.join().unwrap();
        
    } else {
        let Ok(()) = terminal::enable_raw_mode() else {
            anyhow::bail!(
                "Failed to get stdout: if you are trying to pipe 'bandwhich' you should use the --raw flag"
            )
        };

        let mut stdout = std::io::stdout();
        // Ignore enteralternatescreen error
        let _ = crossterm::execute!(&mut stdout, terminal::EnterAlternateScreen);
        let terminal_backend = CrosstermBackend::new(stdout);
        //let terminal_backend_clone = CrosstermBackend::new(std::io::stdout());
        

        let backend = thread::spawn(move || {
            start(terminal_backend, os_input, opts);
        });

        tauri::Builder::default()
            .invoke_handler(tauri::generate_handler![greet, gpl, gcl, gral, gpr, gcr, grar, gpt, gct, grat])
            .run(tauri::generate_context!())
            .expect("error while running tauri application");
        
        backend.join().unwrap();
    }
    
    
    Ok(())
}

pub struct OpenSockets {
    sockets_to_procs: HashMap<LocalSocket, ProcessInfo>,
}

pub struct OsInputOutput {
    pub interfaces_with_frames: Vec<(NetworkInterface, Box<dyn DataLinkReceiver>)>,
    pub get_open_sockets: fn() -> OpenSockets,
    pub terminal_events: Box<dyn Iterator<Item = Event> + Send>,
    pub dns_client: Option<dns::Client>,
    pub write_to_stdout: Box<dyn FnMut(String) + Send>,
}

pub fn start<B>(terminal_backend: B, os_input: OsInputOutput, opts: Opt)
where
    B: Backend + Send + 'static,
{
    
    
    
    // init refresh_rate
    let refresh_rate = Duration::from_millis(opts.refresh_rate);
    

    let running = Arc::new(AtomicBool::new(true));
    let paused = Arc::new(AtomicBool::new(false));
    let last_start_time = Arc::new(RwLock::new(Instant::now()));
    let cumulative_time = Arc::new(RwLock::new(Duration::new(0, 0)));
    let ui_offset = Arc::new(AtomicUsize::new(0));
    let dns_shown = opts.show_dns;

    let mut active_threads = vec![];

    let terminal_events = os_input.terminal_events;
    let get_open_sockets = os_input.get_open_sockets;
    let mut write_to_stdout = os_input.write_to_stdout;
    let mut dns_client = os_input.dns_client;

    let raw_mode = opts.raw;

    let network_utilization = Arc::new(Mutex::new(Utilization::new()));
    
    let ui = Arc::new(Mutex::new(Ui::new(terminal_backend, &opts)));

    let display_handler = thread::Builder::new()
        .name("display_handler".to_string())
        .spawn({
            let running = running.clone();
            let paused = paused.clone();
            let ui_offset = ui_offset.clone();

            let network_utilization = network_utilization.clone();
            let last_start_time = last_start_time.clone();
            let cumulative_time = cumulative_time.clone();
            let ui = ui.clone();

            move || {
                while running.load(Ordering::Acquire) {
                    let render_start_time = Instant::now();
                    let utilization = { network_utilization.lock().unwrap().clone_and_reset() };
                    let OpenSockets { sockets_to_procs } = get_open_sockets();
                    let mut ip_to_host = IpTable::new();
                    if let Some(dns_client) = dns_client.as_mut() {
                        ip_to_host = dns_client.cache();
                        let unresolved_ips = utilization
                            .connections
                            .keys()
                            .filter(|conn| !ip_to_host.contains_key(&conn.remote_socket.ip))
                            .map(|conn| conn.remote_socket.ip)
                            .collect::<Vec<_>>();
                        dns_client.resolve(unresolved_ips);
                    }
                    {
                        let mut ui = ui.lock().unwrap();
                        let paused = paused.load(Ordering::SeqCst);
                        let ui_offset = ui_offset.load(Ordering::SeqCst);
                        if !paused {
                            ui.update_state(sockets_to_procs, utilization, ip_to_host);
                            if opts.alert>0 {
                                ui.check_alerts(opts.alert);
                            }
                        }
                        let elapsed_time = elapsed_time(
                            *last_start_time.read().unwrap(),
                            *cumulative_time.read().unwrap(),
                            paused,
                        );

                        if raw_mode {
                            ui.output_text(&mut write_to_stdout);
                        } else {
                            ui.draw(paused, dns_shown, elapsed_time, ui_offset);
                            ui.output_process_data_to_file("process_record.csv");
                            ui.output_connections_data_to_file("connection_record.csv");
                            ui.output_remote_addresses_data_to_file("remote_addresses_record.csv");
                            ui.output_process_total_data_to_file("process_total_record.csv");
                            ui.output_connections_total_data_to_file("connection_total_record.csv");
                            ui.output_remote_addresses_total_data_to_file("remote_addresses_total_record.csv");
                            let one_month_ago = Utc::now() - Duration::from_secs(60 * 60 * 24 * 90);
                            ui.data_collector.save_process_rate_data(one_month_ago);
                            ui.data_collector.save_connection_rate_data(one_month_ago);
                            ui.data_collector.save_remote_address_rate_data(one_month_ago);
                            ui.data_collector.save_process_total_data(one_month_ago);
                            ui.data_collector.save_connection_total_data(one_month_ago);
                            ui.data_collector.save_remote_address_total_data(one_month_ago);
                            let mut proc_list = PROCESS_LIST.lock().unwrap();
                            *proc_list = ui.data_collector.get_process_list();
                            let mut conn_list = CONNECTION_LIST.lock().unwrap();
                            *conn_list = ui.data_collector.get_connection_list();
                            let mut remote_address_list = REMOTE_ADDRESS_LIST.lock().unwrap();
                            *remote_address_list = ui.data_collector.get_remote_address_list();

                            let mut proc_rates = PROCESS_RATES.lock().unwrap();
                            *proc_rates = ui.data_collector.get_process_rates();
                            let mut conn_rates = CONNECTION_RATES.lock().unwrap();
                            *conn_rates = ui.data_collector.get_connection_rates();
                            let mut remote_address_rates = REMOTE_ADDRESS_RATES.lock().unwrap();
                            *remote_address_rates = ui.data_collector.get_remote_address_rates();

                            let mut proc_total = PROCESS_TOTALS.lock().unwrap();
                            *proc_total = ui.data_collector.get_process_totals();
                            let mut conn_total = CONNECTION_TOTALS.lock().unwrap();
                            *conn_total = ui.data_collector.get_connection_totals();
                            let mut remote_address_total = REMOTE_ADDRESS_TOTALS.lock().unwrap();
                            *remote_address_total = ui.data_collector.get_remote_address_totals();
                        }
                    }
                    let render_duration = render_start_time.elapsed();
                    if render_duration < refresh_rate {
                        park_timeout(refresh_rate - render_duration);
                    }
                }
                if !raw_mode {
                    let mut ui = ui.lock().unwrap();
                    ui.end();
                }
            }
        })
        .unwrap();

    let terminal_event_handler = thread::Builder::new()
        .name("terminal_events_handler".to_string())
        .spawn({
            let running = running.clone();
            let display_handler = display_handler.thread().clone();

            move || {
                for evt in terminal_events {
                    let mut ui = ui.lock().unwrap();

                    match evt {
                        Event::Resize(_x, _y) if !raw_mode => {
                            let paused = paused.load(Ordering::SeqCst);
                            ui.draw(
                                paused,
                                dns_shown,
                                elapsed_time(
                                    *last_start_time.read().unwrap(),
                                    *cumulative_time.read().unwrap(),
                                    paused,
                                ),
                                ui_offset.load(Ordering::SeqCst),
                            );
                        }
                        Event::Key(KeyEvent {
                            modifiers: KeyModifiers::CONTROL,
                            code: KeyCode::Char('c'),
                            kind: KeyEventKind::Press,
                            ..
                        })
                        | Event::Key(KeyEvent {
                            modifiers: KeyModifiers::NONE,
                            code: KeyCode::Char('q'),
                            kind: KeyEventKind::Press,
                            ..
                        }) => {
                            running.store(false, Ordering::Release);
                            display_handler.unpark();
                            match terminal::disable_raw_mode() {
                                Ok(_) => {}
                                Err(_) => println!("Error could not disable raw input"),
                            }
                            let mut stdout = std::io::stdout();
                            if crossterm::execute!(&mut stdout, terminal::LeaveAlternateScreen)
                                .is_err()
                            {
                                println!("Error could not leave alternte screen");
                            };
                            break;
                        }
                        Event::Key(KeyEvent {
                            modifiers: KeyModifiers::NONE,
                            code: KeyCode::Char(' '),
                            kind: KeyEventKind::Press,
                            ..
                        }) => {
                            let restarting = paused.fetch_xor(true, Ordering::SeqCst);
                            if restarting {
                                *last_start_time.write().unwrap() = Instant::now();
                            } else {
                                let last_start_time_copy = *last_start_time.read().unwrap();
                                let current_cumulative_time_copy = *cumulative_time.read().unwrap();
                                let new_cumulative_time =
                                    current_cumulative_time_copy + last_start_time_copy.elapsed();
                                *cumulative_time.write().unwrap() = new_cumulative_time;
                            }

                            display_handler.unpark();
                        }
                        Event::Key(KeyEvent {
                            modifiers: KeyModifiers::NONE,
                            code: KeyCode::Tab,
                            kind: KeyEventKind::Press,
                            ..
                        }) => {
                            let paused = paused.load(Ordering::SeqCst);
                            let elapsed_time = elapsed_time(
                                *last_start_time.read().unwrap(),
                                *cumulative_time.read().unwrap(),
                                paused,
                            );
                            let table_count = ui.get_table_count();
                            let new = ui_offset.load(Ordering::SeqCst) + 1 % table_count;
                            ui_offset.store(new, Ordering::SeqCst);
                            ui.draw(paused, dns_shown, elapsed_time, new);
                        }
                        _ => (),
                    };
                }
            }
        })
        .unwrap();

    active_threads.push(display_handler);
    active_threads.push(terminal_event_handler);

    let sniffer_threads = os_input
        .interfaces_with_frames
        .into_iter()
        .map(|(iface, frames)| {
            let name = format!("sniffing_handler_{}", iface.name);
            let running = running.clone();
            let show_dns = opts.show_dns;
            let network_utilization = network_utilization.clone();

            thread::Builder::new()
                .name(name)
                .spawn(move || {
                    let mut sniffer = Sniffer::new(iface, frames, show_dns);

                    while running.load(Ordering::Acquire) {
                        if let Some(segment) = sniffer.next() {
                            network_utilization.lock().unwrap().update(segment);
                        }
                    }
                })
                .unwrap()
        })
        .collect::<Vec<_>>();
    active_threads.extend(sniffer_threads);

    for thread_handler in active_threads {
        thread_handler.join().unwrap()
    }

    
}




