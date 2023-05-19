//! # Async parser
//!
//! Bitpanda CSV parser with async support for tokio

use crate::Trade;

use csv_async::AsyncReaderBuilder;
use futures::stream::StreamExt;
use tempfile::NamedTempFile;
use tokio::fs::File;
use tokio::io::AsyncBufReadExt;
use tokio::io::AsyncWriteExt;

const BITPANDA_CSV_COL_HEADER: &str = r#""Transaction ID",Timestamp,"Transaction Type",In/Out,"Amount Fiat",Fiat,"Amount Asset",Asset,"Asset market price","Asset market price currency","Asset class","Product ID",Fee,"Fee asset",Spread,"Spread Currency""#;

pub struct AsyncBitpandaTradeParser;

impl AsyncBitpandaTradeParser {
    /// Parse CSV from file at path after sanitizing it
    pub async fn parse(reader: impl AsyncBufReadExt + Unpin) -> csv::Result<Vec<Trade>> {
        debug!("parsing CSV...");
        let sanitized_csv = Self::sanitize_csv(reader).await?;
        debug!("parsing CSV data from {}", sanitized_csv.path().display());
        let file = File::open(sanitized_csv.path()).await?;
        debug!("tempfile opened");
        let mut reader = AsyncReaderBuilder::new()
            .delimiter(b',')
            .create_deserializer(file);
        let mut trades: Vec<Trade> = Vec::new();
        let mut records = reader.deserialize::<Trade>();
        while let Some(Ok(trade)) = records.next().await {
            debug!("found trade {:?}", trade);
            trades.push(trade);
        }
        info!("found {} trades in CSV file", trades.len());
        Ok(trades)
    }

    /// Sanitize the trades csv keeping only the lines after the columns headers
    async fn sanitize_csv(reader: impl AsyncBufReadExt + Unpin) -> csv::Result<NamedTempFile> {
        // open tempfile
        debug!("opening tempfile");
        let work_file = NamedTempFile::new()?;
        let mut writer = File::create(work_file.path()).await?;
        debug!("tempfile opened at {}", work_file.path().display());
        // iter reader lines
        let mut keep = false;
        let mut lines = reader.lines();
        while let Some(line) = lines.next_line().await? {
            if line == BITPANDA_CSV_COL_HEADER {
                debug!("found column header");
                keep = true;
            }
            if keep {
                writer.write_all(line.as_bytes()).await?;
                writer.write(&[b'\n']).await?;
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
    use tokio::io::BufReader;

    #[tokio::test]
    async fn should_parse_csv_data_async() {
        let file = File::open(Path::new("./test/bitpanda.csv")).await.unwrap();
        let trades = AsyncBitpandaTradeParser::parse(BufReader::new(file))
            .await
            .unwrap();
        assert_eq!(trades.len(), 12);
    }
}
