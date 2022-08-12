use crate::modem::at_command::{ATParseErr, ATParseLine};
use crate::util::collect_array;

/// Indicates whether the app network is active
#[derive(Debug)]
pub struct IncomingConnection {
    // core::net::IpAddr doesn't exist, very sad.
    // TODO: find out if modem supports ipv6
    remote_ip: [u8; 4],
}

impl ATParseLine for IncomingConnection {
    fn from_line(line: &str) -> Result<Self, ATParseErr> {
        let (message, ip) = line.split_once(": ").ok_or(ATParseErr)?;
        if message != "REMOTE IP" {
            return Err(ATParseErr);
        }

        Ok(IncomingConnection {
            remote_ip: collect_array(ip.splitn(4, '.').filter_map(|segment| segment.parse().ok()))
                .ok_or(ATParseErr)?,
        })
    }
}
