//! # Parser
//!
//! Bitpanda CSV parser

use crate::Trade;

use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};
use tempfile::NamedTempFile;

const BITPANDA_CSV_COL_HEADER: &str = r#""Transaction ID",Timestamp,"Transaction Type",In/Out,"Amount Fiat",Fiat,"Amount Asset",Asset,"Asset market price","Asset market price currency","Asset class","Product ID",Fee,"Fee asset",Spread,"Spread Currency""#;

pub struct BitpandaTradeParser;

impl BitpandaTradeParser {
    /// Parse CSV from file at path after sanitizing it
    pub fn parse(reader: impl Read) -> csv::Result<Vec<Trade>> {
        debug!("parsing CSV...");
        let sanitized_csv = Self::sanitize_csv(reader)?;
        debug!("parsing CSV data from {}", sanitized_csv.path().display());
        let file = File::open(sanitized_csv.path())?;
        debug!("tempfile opened");
        let mut reader = csv::Reader::from_reader(file);
        let mut trades: Vec<Trade> = Vec::new();
        for trade in reader.deserialize::<Trade>() {
            let trade = trade?;
            debug!("found trade {:?}", trade);
            trades.push(trade);
        }
        info!("found {} trades in CSV file", trades.len());
        Ok(trades)
    }

    /// Sanitize the trades csv keeping only the lines after the columns headers
    fn sanitize_csv(reader: impl Read) -> csv::Result<NamedTempFile> {
        // open tempfile
        debug!("opening tempfile");
        let work_file = NamedTempFile::new()?;
        let mut writer = File::create(work_file.path())?;
        debug!("tempfile opened at {}", work_file.path().display());
        // iter reader lines
        let mut keep = false;
        for line in BufReader::new(reader).lines() {
            let line = line?;
            if line == BITPANDA_CSV_COL_HEADER {
                debug!("found column header");
                keep = true;
            }
            if keep {
                writeln!(writer, "{}", line)?;
            }
        }
        debug!("csv file sanitized");
        Ok(work_file)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    use pretty_assertions::assert_eq;
    use std::path::Path;

    #[test]
    fn should_parse_csv_data() {
        let reader = File::open(Path::new("./test/bitpanda.csv")).unwrap();
        let trades = BitpandaTradeParser::parse(reader).unwrap();
        assert_eq!(trades.len(), 12);
    }
}
