///! # Arguments Module
///! Read in command line arguments using clap
///! For the moment this is limited to:
///! - infile TOML configuration file
///! - outfile: simulation result
///! - silent: boolean
use clap::{App, Arg};

/// Command line arguments struct, infile, outfile, and silent (i.e. emit to stdout)
pub struct Args {
    /// path to input TOML configuation file for running a calculation
    pub infile: String,
    /// path to JSON output file for saving the results of calculation
    pub outfile: Option<String>,
    /// Boolean for remaining silent or writing to stdout
    pub silent: bool,

    pub demo: bool,
}

impl Args {
    /// Parse command line arguments
    pub fn parse() -> Self {
        let matches = App::new("magnet_rs")
            .author("Peter Dunne")
            .version("0.2.2")
            .about("Calculates 2D magnetic fields")
            .arg(
                Arg::with_name("infile")
                    .help("Read from a toml file")
                    .short("i")
                    .long("infile")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("outfile")
                    .short("o")
                    .long("outfile")
                    .takes_value(true)
                    .help("Write simulation output to a file"),
            )
            .arg(Arg::with_name("silent").short("s").long("silent").help("Silences console output, except for the progress bar"))
            .arg(
                Arg::with_name("demo")
                    .short("d")
                    .long("demo")
                    .help("Runs an example calculation of two square magnets and saves it to example_out.json."),
            )
            .get_matches();

        let infile = matches.value_of("infile").unwrap_or_default().to_string();

        let outfile = if matches.is_present("outfile") {
            Some(matches.value_of("outfile").unwrap_or_default().to_string())
        } else {
            None
        };

        let demo = matches.is_present("demo");

        let silent = matches.is_present("silent");
        Self {
            infile,
            outfile,
            silent,
            demo,
        }
    }
}
