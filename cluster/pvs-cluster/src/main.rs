mod network;
use network::{get_local_ips, ipstr_starts_with};

use clap::{Arg, Command, ArgAction};

fn main() {

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
            .value_parser(["list_interfaces", "list_ips"])
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
        ).get_matches();

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
                if verbose {println!("IPv4 Addresses for {}:", name);}
                for ip in ips.ipv4_addrs {
                    if name == ip.name.unwrap_or_default() {
                        if ! ipstr_starts_with(& ip.ip, & starting_octets){
                            continue;
                        }
                        if verbose {
                            println!(" - {}", ip.ip);
                        } else {
                            println!("{}", ip.ip);
                        }
                    }
                }
            }

            if print_v6 {
                if verbose {println!("IPv6 Addresses for {}:", name);}
                for ip in ips.ipv6_addrs {
                    if name == ip.name.unwrap_or_default() {
                        if ! ipstr_starts_with(& ip.ip, & starting_octets){
                            continue;
                        }
                        if verbose {
                            println!(" - {}", ip.ip);
                        } else {
                            println!("{}", ip.ip);
                        }
                    }
                }
            }
        }

        &_ => todo!()
    }
}