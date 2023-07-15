use std::io::{BufRead, BufReader, BufWriter, Write, self};
use std::fs::{File, OpenOptions};
use regex::Regex;
use clap::{Arg, App};

fn parse_line(line_number: u32, line: String, re: &Regex, writer: &mut BufWriter<File>) -> std::io::Result<bool> {
    if let Some(caps) = re.captures(&line) {
        let start_time = &caps["start"].replace(".", ",");
        let end_time = &caps["end"].replace(".", ",");
        let text = &caps["text"];

        write!(writer, "{}\n{} --> {}\n{} \n\n", line_number, start_time, end_time, text)?;
        writer.flush()?;
        return Ok(true)
    } else {
        return Ok(false)
    }
}

fn main() -> std::io::Result<()> {
    let re = Regex::new(r"\[(?P<start>[\d:.]+) --> (?P<end>[\d:.]+)\]\s*(?P<text>.*)").unwrap();

    let matches = App::new("whispersub")
        .version("1.0")
        .about("Transforms text coming out of Whisper to SRT format")
        .arg(Arg::with_name("INPUT")
             .help("Sets the input file to use")
             .index(1))
        .arg(Arg::with_name("output")
             .short("o")
             .long("output")
             .help("Sets the output file to use")
             .takes_value(true))
        .get_matches();

    let output_file_name = matches.value_of("output").unwrap_or("output.srt");

    let output_file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(&output_file_name)?;

    let mut writer = BufWriter::new(output_file);
    let mut line_number = 1;
    
    if let Some(input_file_name) = matches.value_of("INPUT") {
        let file = File::open(input_file_name)?;
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let line = line?;
            if parse_line(line_number, line, &re, &mut writer)? {
                line_number += 1;
            }
            
        }
    } else {
        let stdin = io::stdin();
        let handle = stdin.lock();
        let mut reader = io::BufReader::new(handle);
        let mut buffer = String::new();
        
        loop {
            match reader.read_line(&mut buffer) {
                Ok(bytes_read) =>{
                    if bytes_read == 0 {
                        break;
                    }
                    if parse_line(line_number, buffer.trim_end().to_string(), &re, &mut writer)? {
                        line_number += 1;
                    }
                    buffer.clear();
                }
                Err(err) => panic!("{}", err),
            }
        }
    
    }
    Ok(())
}
