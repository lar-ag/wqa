#![feature(async_await)]

use quicli::prelude::*;
use structopt::StructOpt;
// use syntect::easy::HighlightFile;
// use syntect::highlighting::{Style, ThemeSet};
// use syntect::parsing::SyntaxSet;
// use syntect::util::as_24_bit_terminal_escaped;
// use std::io::BufRead;

// use crate::run::run;
// use crate::verify::verify;
// use notify::DebouncedEvent;
// use notify::{RecommendedWatcher, RecursiveMode, Watcher};
// use std::ffi::OsStr;
// use std::fs;
// use std::path::Path;
// use std::sync::mpsc::channel;
// use std::time::Duration;
// use indicatif::ProgressBar;

mod exercise;
mod run;
mod verify;
use quicktoc::{
    monitor::*,
    Cli,
    error::{MonitorError},
};


gen_boolean_enum!(MyEnum);


async fn get_uv_state() -> Result<Monitor>{

}

async fn read_digital() -> Result<bool,MonitorError> {

}



async fn say_hi() {
    println!("Hello world! 🤖");
}



async fn say_name() {

    println!(r#"  ___ rlc    _      _    _____ ___   ____            "#);
    println!(r#" / _ \ _   _(_) ___| | _|_   _/ _ \ / ___|   ___   __"#);
    println!(r#"| | | | | | | |/ __| |/ / | || | | | |  | | | \ \ / /"#);
    println!(r#"| |_| | |_| | | (__|   <  | || |_| | |__| |_| |\ V / "#);
    println!(r#" \__\_\\__,_|_|\___|_|\_\ |_| \___/ \____\__,_| \_/  "#);
    println!(r#""#);

}
async get_pid() u64  {

}

pub async machine_id() -> Resul<MachreId,MonitorError> {
    MachineId::read()
}

pub async serial() -> Result<String> {

}




fn main() -> CliResult {
    pub GP[6,false];
    let args: Cli =  Cli::from_args();

    let api = async {
        
        let mut monitor = Monitor::new();
        let mut state   = get_uv_state();
    };

    
    // let mut app = api::make_app();
    Ok(())

    // if matches.subcommand_name().is_none() {
    //     let mut highlighter =
    //         HighlightFile::new("default_out.md", &ss, &ts.themes["base16-eighties.dark"]).unwrap();
    //     for maybe_line in highlighter.reader.lines() {
    //         let line = maybe_line.unwrap();
    //         let regions: Vec<(Style, &str)> = highlighter.highlight_lines.highlight(&line, &ss);
    //         println!("{}", as_24_bit_terminal_escaped(&regions[..], true));
    //     }
    // }
    // println!("\x1b[0m");
}



fn  quicktoc_uv() {
     if None == matches.subcommand_name() {
    println!();

    println!(r#"  ___ rlc    _      _    _____ ___   ____            "#);
    println!(r#" / _ \ _   _(_) ___| | _|_   _/ _ \ / ___|   ___   __"#);
    println!(r#"| | | | | | | |/ __| |/ / | || | | | |  | | | \ \ / /"#);
    println!(r#"| |_| | |_| | | (__|   <  | || |_| | |__| |_| |\ V / "#);
    println!(r#" \__\_\\__,_|_|\___|_|\_\ |_| \___/ \____\__,_| \_/  "#);
    println!(r#"🤖");
}