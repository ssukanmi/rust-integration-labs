use std::fmt;

use rand::RngCore;
use smoltcp::wire;

#[derive(Debug)]
pub struct MacAddress([u8; 6]);

impl fmt::Display for MacAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let octet = self.0;
        write!(
            f,
            "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
            octet[0], octet[1], octet[2], octet[3], octet[4], octet[5]
        )
    }
}

impl Default for MacAddress {
    fn default() -> Self {
        Self::new()
    }
}

impl MacAddress {
    pub fn new() -> Self {
        let mut octet = [0; 6];
        rand::rng().fill_bytes(&mut octet);
        octet[0] |= 0b_0000_0010;
        octet[0] &= 0b_1111_1110;
        Self(octet)
    }
}

impl From<MacAddress> for wire::EthernetAddress {
    fn from(mac_addr: MacAddress) -> Self {
        Self(mac_addr.0)
    }
}

// impl Into<wire::EthernetAddress> for MacAddress {
//     fn into(self) -> wire::EthernetAddress {
//         wire::EthernetAddress { 0: self.0 }
//     }
// }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_mac() {
        let mac = MacAddress::new();
        println!("{}", mac);
    }
}
