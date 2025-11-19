use std::{
    error, fmt, io,
    net::{self, SocketAddr},
};

use trust_dns::{proto::error::ProtoError, rr::Name};

#[derive(Debug)]
pub enum DnsError {
    ParseDomainName(ProtoError),
    ParseDnsServerAddress(net::AddrParseError),
    Encoding(ProtoError),
    Decoding(ProtoError),
    Network(io::Error),
    Sending(io::Error),
    Receving(io::Error),
}

impl fmt::Display for DnsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}

impl error::Error for DnsError {}

impl From<net::AddrParseError> for DnsError {
    fn from(error: net::AddrParseError) -> Self {
        DnsError::ParseDnsServerAddress(error)
    }
}

impl From<ProtoError> for DnsError {
    fn from(error: ProtoError) -> Self {
        DnsError::ParseDomainName(error)
    }
}

fn message_id() -> u16 {
    let candidate = rand::random();
    if candidate == 0 {
        return message_id();
    }
    candidate
}

pub fn resolve(
    dns_server_addr: &str,
    domain_name: &str,
) -> Result<Option<net::IpAddr>, Box<dyn error::Error>> {
    let domain_name = Name::from_ascii(domain_name).map_err(DnsError::ParseDomainName)?;
    let dns_server_addr = format!("{}:53", dns_server_addr);
    let dns_server: SocketAddr = dns_server_addr
        .parse()
        .map_err(DnsError::ParseDnsServerAddress)?;

    Ok(None)
}
