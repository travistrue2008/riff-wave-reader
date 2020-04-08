use anyhow::Error;
use structopt::StructOpt;

use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::path::PathBuf;

use byteorder::{ByteOrder, LittleEndian};

use atrac3p::Atrac3Plus;
use riff_wave_reader::RiffWaveReader;

fn main() -> Result<(), Error> {
    let opts = Opts::from_args();

    match opts.command {
        Command::Print { input } => {
            let file = File::open(input)?;
            let reader = BufReader::new(file);

            let reader = RiffWaveReader::new(reader)?;

            reader.print_info();
        }
        Command::Raw { input } => {
            let file = File::open(input)?;
            let reader = BufReader::new(file);

            let mut reader = RiffWaveReader::new(reader)?;

            let data = reader.data()?.collect::<Vec<_>>();

            for x in 0..2048 {
                let byte = data.get(x).unwrap();
                println!("{:08b}", byte);
            }
        }
        Command::Atrac { input } => {
            let file = File::open(input)?;
            let reader = BufReader::new(file);

            let atrac = Atrac3Plus::new(reader)?;

            atrac.into_iter().for_each(drop);
        }
        Command::Play { input } => {
            let file = File::open(input)?;
            let reader = BufReader::new(file);

            let atrac = Atrac3Plus::new(reader)?;

            let device = rodio::default_output_device().unwrap();
            let sink = rodio::Sink::new(&device);

            sink.append(atrac);
            sink.play();
            sink.sleep_until_end();
        }
    }

    Ok(())
}

#[derive(StructOpt)]
#[structopt(name = "riff-cli")]
struct Opts {
    #[structopt(subcommand)]
    command: Command,
}

#[derive(StructOpt)]
enum Command {
    Print {
        #[structopt(parse(from_os_str))]
        input: PathBuf,
    },
    Raw {
        #[structopt(parse(from_os_str))]
        input: PathBuf,
    },
    Atrac {
        #[structopt(parse(from_os_str))]
        input: PathBuf,
    },
    Play {
        #[structopt(parse(from_os_str))]
        input: PathBuf,
    },
}
