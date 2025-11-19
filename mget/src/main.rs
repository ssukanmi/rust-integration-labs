use std::{error::Error, net};

use clap::Parser;
use url::Url;

use crate::args::Args;

mod args;
mod dns;
mod ethernet;
mod http;

fn main() -> Result<(), Box<dyn Error>> {
    let Args {
        url,
        tap_device,
        dns_server,
    } = Args::parse();

    let url = Url::parse(&url)?;

    if url.scheme() != "http" {
        return Err("Error: only HTTP protocol supported".into());
    }

    // let tap =

    let domain_name = url.domain().ok_or("Error: domain name required")?;

    let _dns_server = dns_server.parse::<net::Ipv4Addr>()?;

    let addr = dns::resolve(&dns_server, domain_name)?.ok_or("Error: IpAddr is None")?;

    let mac = ethernet::MacAddress::new();

    // http::get(tap, mac, addr, url)?;

    Ok(())
}
