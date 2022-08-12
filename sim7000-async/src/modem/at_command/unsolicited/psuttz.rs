use crate::modem::at_command::{stub_parser_prefix, ATParseErr, ATParseLine};

// stub type
#[derive(Debug)]
pub struct Psuttz;

impl ATParseLine for Psuttz {
    fn from_line(line: &str) -> Result<Self, ATParseErr> {
        stub_parser_prefix(line, "*PSUTTZ:", Psuttz)
    }
}
