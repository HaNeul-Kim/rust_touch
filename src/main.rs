use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

use chrono::{DateTime, Local, NaiveDateTime, TimeZone};
use filetime::FileTime;

fn print_usage() {
  // multi line string
  println!(r#"Usage: touch <FILE> [<yyyyMMdd_HHmmss.SSS>]"#);
}

/// @version 0.1.0
/// @author HaNeul Kim
/// @since 2022-03-02
fn main() -> std::io::Result<()> {
  let args: Vec<String> = env::args().collect();
  match args.len() {
    2 => {
      let file_path = &args[1];
      let metadata = fs::metadata(file_path)?;
      if let Ok(time) = metadata.modified() {
        let date_time: DateTime<Local> = time.into();
        let old_date_time = date_time.format("%Y%m%d_%H%M%S%.3f");
        println!("{file_path} {old_date_time}");
      }
    }
    3 => {
      // let file_path = &args[1];
      // let time_format = &args[2];
      let (file_path, time_format) = (&args[1], &args[2]);

      let metadata = fs::metadata(file_path)?;
      if let Ok(time) = metadata.modified() {
        let old_date_time: DateTime<Local> = time.into();

        // DateTime: required tz, NaiveDateTime: no required tz
        // let ts = DateTime::parse_from_str(&args[1], "%Y%m%d_%H%M%S%.3f %z")?.timestamp_millis();
        // let ts = NaiveDateTime::parse_from_str(&args[1], "%Y%m%d_%H%M%S%.3f")?.timestamp_millis();
        let naive = NaiveDateTime::parse_from_str(time_format, "%Y%m%d_%H%M%S%.3f").unwrap();
        let new_date_time: DateTime<Local> = Local.from_local_datetime(&naive).unwrap();
        let new_file_time = FileTime::from(SystemTime::from(new_date_time));
        filetime::set_file_mtime(Path::new(file_path), new_file_time);

        println!("{} {} -> {}", file_path, old_date_time.format("%F %T%.3f"), new_date_time.format("%F %T%.3f"));
      } else {
        println!("Not supported on this platform");
      }
    }
    _ => {
      print_usage();
      std::process::exit(1);
    }
  }
  Ok(())
}
