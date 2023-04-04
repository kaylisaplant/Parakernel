
use pnet::datalink;
use pnet::ipnetwork::IpNetwork;
use std::net::IpAddr;


#[derive(Debug)]
pub struct LocalInterface {
    pub ip:   IpAddr,
    pub name: Option<String>
}

#[derive(Debug)]
pub struct LocalIpAddresses {
    pub ipv4_addrs: Vec<LocalInterface>,
    pub ipv6_addrs: Vec<LocalInterface>
}



pub fn get_local_ips() -> LocalIpAddresses {
    let mut ipv4_addrs = Vec::new();
    let mut ipv6_addrs = Vec::new();

    for iface in datalink::interfaces() {
        for ip_network in iface.ips {
            match ip_network {
                IpNetwork::V4(ipv4_network) => {
                    ipv4_addrs.push(
                        LocalInterface {
                            ip:   IpAddr::V4(ipv4_network.ip()),
                            name: Some(iface.name.clone())
                        }
                    )
                }
                IpNetwork::V6(ipv6_network) => {
                    ipv6_addrs.push(
                        LocalInterface {
                            ip:   IpAddr::V6(ipv6_network.ip()),
                            name: Some(iface.name.clone())
                        }
                    )
                }
            }
        }
    }
    LocalIpAddresses {
        ipv4_addrs: ipv4_addrs,
        ipv6_addrs: ipv6_addrs
    }
}


pub fn ipstr_starts_with(
        ip: & IpAddr, starting_octets: & Option<& String>
    ) -> bool {
    match * starting_octets {
        Some(start) => ip.to_string().starts_with(start),
        None => false,
    }
}