#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;
use calamine::{open_workbook_auto, DataType, Range, Reader};
//use napi::Error;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
//use chrono::{DateTime, Utc};

#[napi]
pub async fn make_csv(file: String, separator: Option<String>) {
  println!("Создаём файлы");
  //Первый файл
  let sce = PathBuf::from(&file);
  match sce.extension().and_then(|s: &std::ffi::OsStr| s.to_str()) {
    Some("xlsx") | Some("xlsm") | Some("xlsb") | Some("xls") => (),
    _ => panic!("Expecting an excel file"),
  }

  let dest: PathBuf = sce.with_extension("csv");
  let mut dest = BufWriter::new(File::create(dest).unwrap());
  let mut xl = open_workbook_auto(&file).unwrap();
  let range = xl.worksheet_range_at(0).unwrap().unwrap();
  let sep = separator.unwrap_or(','.to_string());
  write_range(&mut dest, &range, sep).await.unwrap();
}

async fn write_range<W: Write>(dest: &mut W, range: &Range<DataType>, separator: String) -> std::io::Result<()> {
  let n = range.get_size().1 - 1;
  for r in range.rows() {
    for (i, c) in r.iter().enumerate() {
      match *c {
        DataType::Empty => Ok(()),
        DataType::String(ref s) | DataType::DateTimeIso(ref s) | DataType::DurationIso(ref s) => {
          write!(dest, "{}", s)
        }
        DataType::DateTime(ref s) | DataType::Duration(ref s) => {write!(dest, "{}", s.to_string())}
        DataType::Float(ref f) => {
          write!(dest, "{}", f)
        }

        DataType::Int(ref i) => write!(dest, "{}", i),
        DataType::Error(ref e) => write!(dest, "{:?}", e),
        DataType::Bool(ref b) => write!(dest, "{}", b),
      }?;
      if i != n {
        write!(dest, "{}", separator)?;
      }
    }
    write!(dest, "\r\n")?;
  }
  Ok(())
}
