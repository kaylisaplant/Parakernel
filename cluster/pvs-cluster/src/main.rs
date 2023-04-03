mod network;
use network::get_local_ips;

use clap::{Arg, Command};

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
        .get_matches();

    let ips = get_local_ips();

    let operation = args.get_one::<String>("operation").unwrap();
    match operation.as_str() {
        "list_interfaces" => {
            let mut ipv4_names = Vec::new();
            let mut ipv6_names = Vec::new();

            println!("IPv4 Interfaces:");
            for ip in ips.ipv4_addrs {
                let name: &String = &ip.name.unwrap_or_default();
                if ! ipv4_names.contains(name) {
                    println!(" - {}", name);
                    ipv4_names.push(name.to_string());
                }
            }

            println!("IPv6 Interfaces:");
            for ip in ips.ipv6_addrs {
                let name: &String = &ip.name.unwrap_or_default();
                if ! ipv6_names.contains(name) {
                    println!(" - {}", name);
                    ipv6_names.push(name.to_string());
                }
            }
        }

        "list_ips" => {
            assert!(args.contains_id("interface_name"));
            let name = args.get_one::<String>("interface_name").unwrap().as_str();
            
            println!("IPv4 Addresses for {}:", name);
            for ip in ips.ipv4_addrs {
                if name == ip.name.unwrap_or_default() {
                    println!(" - {}", ip.ip)
                }
            }

            println!("IPv6 Addresses for {}:", name);
            for ip in ips.ipv6_addrs {
                if name == ip.name.unwrap_or_default() {
                    println!(" - {}", ip.ip)
                }
            }
        }

        &_ => todo!()
    }
}