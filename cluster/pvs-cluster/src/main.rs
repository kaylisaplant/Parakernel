mod network;
use network::{get_local_ips, get_matching_ipstr};

mod connection;
use connection::{cread, cwrite, Addr, server};

mod service;
use service::{Payload, State, serialize, request_handler};

use clap::{Arg, Command, ArgAction};
use std::time::{SystemTime, UNIX_EPOCH};
use std::net::TcpStream;

fn unix_timestamp() -> u64 {
    let now = SystemTime::now();
    let since_epoch = now.duration_since(UNIX_EPOCH).unwrap();
    since_epoch.as_secs()
}


fn main() -> std::io::Result<()> {

    let args = Command::new("ParaView Server Cluster")
        .version("1.0")
        .author("Johannes Blaschke")
        .about("Manages a cluster of ParaView Servers")
        .arg(
            Arg::new("operation")
            .short('o')
            .long("operation")
            .value_name("OPERATION")
            .help("Operation to be performed")
            .num_args(1)
            .required(true)
            .value_parser(["list_interfaces", "list_ips", "listen", "claim"])
        )
        .arg(
            Arg::new("interface_name")
            .short('n')
            .long("name")
            .value_name("NAME")
            .help("Interface Name")
            .num_args(1)
            .required(false)
        )
        .arg(
            Arg::new("ip_start")
            .short('i')
            .long("ip-start")
            .value_name("STARTING OCTETS")
            .help("Only return ip addresses whose starting octets match these.")
            .num_args(1)
            .required(false)
        )
        .arg(
            Arg::new("ip_version")
            .long("ip-version")
            .value_name("IP VERSION")
            .help("Output results only matching this IP version")
            .num_args(1)
            .required(false)
            .value_parser(clap::value_parser!(i32))
        )
        .arg(
            Arg::new("verbose")
            .short('v')
            .long("verbose")
            .help("Don't output headers")
            .num_args(0)
            .required(false)
            .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("host")
            .long("host")
            .value_name("HOST")
            .help("Host to bind to")
            .num_args(1)
            .required(false)
        )
        .arg(
            Arg::new("port")
            .long("port")
            .value_name("PORT")
            .help("Port to bind server and client to.")
            .num_args(1)
            .required(false)
            .value_parser(clap::value_parser!(i32))
        )
        .arg(
            Arg::new("key")
            .long("key")
            .value_name("KEY")
            .help("Service access key")
            .num_args(1)
            .required(false)
            .value_parser(clap::value_parser!(u64))
        )
        .get_matches();

    let ips = get_local_ips();

    let ip_version =   args.get_one::<i32>("ip_version");
    let verbose    = * args.get_one::<bool>("verbose").unwrap();
    let mut print_v4 = false;
    let mut print_v6 = false;
    if ip_version.is_some() {
        match * ip_version.unwrap() {
            4 => print_v4 = true,
            6 => print_v6 = true,
            _ => panic!(
                "Please specify IP version 4 or 6, or ommit `--ip-version` for both."
            )
        }
    } else {
        print_v4 = true;
        print_v6 = true;
    }

    let operation = args.get_one::<String>("operation").unwrap();
    match operation.as_str() {
        "list_interfaces" => {
            let mut ipv4_names = Vec::new();
            let mut ipv6_names = Vec::new();

            if print_v4 {
                if verbose {println!("IPv4 Interfaces:");}
                for ip in ips.ipv4_addrs {
                    let name: & String = & ip.name.unwrap_or_default();
                    if ! ipv4_names.contains(name) {
                        if verbose {
                            println!(" - {}", name);
                        } else {
                            println!("{}", name);
                        }
                        ipv4_names.push(name.to_string());
                    }
                }
            }

            if print_v6 {
                if verbose {println!("IPv6 Interfaces:");}
                for ip in ips.ipv6_addrs {
                    let name: & String = & ip.name.unwrap_or_default();
                    if ! ipv6_names.contains(name) {
                        if verbose {
                            println!(" - {}", name);
                        } else {
                            println!("{}", name);
                        }
                        ipv6_names.push(name.to_string());
                    }
                }
            }
        }

        "list_ips" => {
            assert!(args.contains_id("interface_name"));
            let name = args.get_one::<String>("interface_name").unwrap().as_str();
            let starting_octets = args.get_one::<String>("ip_start");

            if print_v4 {
                let ipstr = get_matching_ipstr(
                    & ips.ipv4_addrs, name, & starting_octets
                );
                if verbose {println!("IPv4 Addresses for {}:", name);}
                for ip in ipstr {
                    if verbose {
                        println!(" - {}", ip);
                    } else {
                        println!("{}", ip);
                    }
                }
            }

            if print_v6 {
                let ipstr = get_matching_ipstr(
                    & ips.ipv6_addrs, name, & starting_octets
                );
                if verbose {println!("IPv6 Addresses for {}:", name);}
                for ip in ipstr {
                    if verbose {
                        println!(" - {}", ip);
                    } else {
                        println!("{}", ip);
                    }
                }
            }
        }

        "listen" => {
            assert!(args.contains_id("host"));
            assert!(args.contains_id("port"));

            let mut state: State = State::new();
            let mut handler =  |stream: &mut TcpStream| {
                return request_handler(&mut state, stream);
            };

            let addr = Addr {
                host:   args.get_one::<String>("host").unwrap(),
                port: * args.get_one::<i32>("port").unwrap()
            };

            server(& addr, handler);
            // let rec = cread(host, port)?;
            // println!("REC: {:?}", rec);
        }

        "claim" => {
            assert!(args.contains_id("host"));
            assert!(args.contains_id("port"));
            assert!(args.contains_id("interface_name"));
            assert!(args.contains_id("key"));

            let host =   args.get_one::<String>("host").unwrap().as_str();
            let port = * args.get_one::<i32>("port").unwrap();
            let key  = * args.get_one::<u64>("key").unwrap();

            let name = args.get_one::<String>("interface_name").unwrap().as_str();
            let starting_octets = args.get_one::<String>("ip_start");
            let (ipstr, all_ipstr) = if print_v4 {
                (get_matching_ipstr(& ips.ipv4_addrs, name, & starting_octets),
                get_matching_ipstr(& ips.ipv4_addrs, name, & None))
            } else {
                (get_matching_ipstr(& ips.ipv6_addrs, name, & starting_octets),
                get_matching_ipstr(& ips.ipv6_addrs, name, & None))
            };
            let payload = serialize(& Payload {
                service_addr: ipstr,
                service_port: port,
                service_claim: unix_timestamp(),
                interface_addr: all_ipstr,
                key: key
            });
            let _rec = cwrite(host, port, & payload);
        }

        &_ => todo!()
    }

    Ok(())
}