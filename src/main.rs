use log::{debug, info};
use native_dialog::FileDialog;
use regex::Regex;
use simplelog::{Config, LevelFilter, WriteLogger};
use std::env::temp_dir;
use std::ffi::OsStr;
use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::{exit, Command};
use std::str::from_utf8;

const FFMPEG_EXECUTABLE: &[u8] = include_bytes!("../ffmpeg.exe");

fn main() {
    let logfile_path = temp_dir().join("autosplitter.log");
    eprintln!("Logfile path: {}", logfile_path.display());
    WriteLogger::init(
        LevelFilter::Debug,
        Config::default(),
        File::create(logfile_path).expect("Could not create log file."),
    )
    .expect("Could not initialize logger.");

    let ffmpeg_path = temp_dir().join("ffmpeg.exe");
    info!("ffmpeg path: {}", ffmpeg_path.display());

    if ffmpeg_path.exists() {
        info!("ffmpeg already extracted")
    } else {
        eprint!("Extracting ffmpeg...");
        let mut file =
            File::create(ffmpeg_path.clone()).expect("Failed to create temporary file for ffmpeg.");
        file.write_all(FFMPEG_EXECUTABLE)
            .expect("Failed to write bytes to ffmpeg temporary file.");
        eprintln!(" done");
    }

    let source_file = read_file(
        "Input file path (or type 'd' to open a system file dialog): ",
        false,
    );

    let target_folder = read_file(
        "Target folder path (or type 'd' to open a system file dialog): ",
        true,
    );

    let bpm = read_int("BPM: ", None);
    let samples = read_int("Total Samples [256]: ", Some(256));
    let fadeout_time = read_int(
        "Fadeout time at the end of each beat, in milliseconds [10]: ",
        Some(10),
    );
    let output_duration_seconds = samples as f64 / (bpm as f64 * 4.) * 60.;

    let probe_output = run_ffmpeg(
        &ffmpeg_path,
        &[
            "-hide_banner",
            "-i",
            source_file.to_str().unwrap(),
            "-f",
            "null",
            "-",
        ],
    );

    let duration_seconds =
        parse_duration_seconds(&probe_output).expect("Could not parse ffmpeg output.");

    if output_duration_seconds > duration_seconds {
        println!("Not enough input samples to produce desired output.");
        pause(1);
    }

    print!(
        "Splitting into {} sixteenth-note chunks at {}BPM... ",
        samples, bpm
    );

    io::stdout().flush().unwrap();
    split(
        source_file,
        target_folder,
        ffmpeg_path,
        bpm,
        samples,
        (fadeout_time as f64) / 1000.,
    );
    pause(0);
}

fn print_progress(sample: u32, samples: u32) {
    let samplenum = make_sample_number_string(sample, samples);

    if sample != 0 {
        print!(
            "{}",
            (8u8 as char).to_string().repeat(samplenum.len() * 2 + 3)
        );
    }

    if sample + 1 == samples {
        println!("done");
        return;
    }

    print!("[{}/{}]", samplenum, samples);

    io::stdout().flush().unwrap();
}

fn split(
    source_file: PathBuf,
    target_folder: PathBuf,
    ffmpeg_path: PathBuf,
    bpm: u32,
    samples: u32,
    fade_time: f64,
) {
    let mut start_ts = 0.;
    let increment = 60. / (bpm as f64 * 4.);

    for sample in 0..samples {
        let target_file = make_target_file(&source_file, &target_folder, sample, samples);
        debug!(
            "Splitting to {}: {}, {}",
            target_file.display(),
            start_ts.to_string(),
            increment.to_string()
        );

        run_ffmpeg(
            &ffmpeg_path,
            &[
                "-hide_banner",
                "-ss",
                &start_ts.to_string(),
                "-i",
                source_file.to_str().unwrap(),
                "-t",
                &increment.to_string(),
                "-af",
                &format!("afade=t=out:ss={}:d={}", increment - fade_time, increment),
                target_file.to_str().unwrap(),
            ],
        );

        print_progress(sample, samples);

        start_ts = increment * (sample + 1) as f64;
    }
}

fn make_target_file(
    source_file: &PathBuf,
    target_folder: &PathBuf,
    sample: u32,
    samples: u32,
) -> PathBuf {
    let base = source_file
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    let ext = source_file.extension().unwrap().to_str().unwrap();

    let samplenum = make_sample_number_string(sample, samples);

    target_folder.join(PathBuf::from(base + "_" + &*samplenum + "." + ext))
}

fn make_sample_number_string(sample: u32, samples: u32) -> String {
    let digits = samples.to_string().len();
    let samplenum = (sample + 1).to_string();
    return "0".repeat(digits - samplenum.len()).to_string() + &samplenum;
}

fn run_ffmpeg<I, S>(ffmpeg_path: &PathBuf, args: I) -> String
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let ffmpeg = Command::new(ffmpeg_path.to_str().unwrap())
        .args(args)
        .output()
        .expect("Failed to create process.");
    let stderr = from_utf8(&ffmpeg.stderr).unwrap();
    debug!("{}", stderr);
    stderr.to_string()
}

fn read_file(message: &str, folder: bool) -> PathBuf {
    loop {
        print!("{}", message);
        let input = read_line();

        if input == "d" {
            let response;

            if folder {
                response = FileDialog::new().show_open_single_dir();
            } else {
                response = FileDialog::new().show_open_single_file()
            }

            match response.expect("Failed to create file dialog.") {
                Some(path) => return path,
                None => println!("No file selected, please try again."),
            }
        } else {
            let path = PathBuf::from(input.clone());
            if path.is_file() {
                return path;
            }

            println!("{} is not a valid file, please try again.", input);
        }
    }
}

fn read_int(message: &str, default: Option<u32>) -> u32 {
    loop {
        print!("{}", message);
        let input = read_line();

        if input.is_empty() && default.is_some() {
            return default.unwrap();
        }

        match input.parse() {
            Ok(i) => return i,
            Err(_) => println!("Could not parse as integer, please try again."),
        }
    }
}

fn read_line() -> String {
    io::stdout().flush().unwrap();
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line from stdin.");
    buffer.trim().to_string()
}

fn parse_duration_seconds(probe_output: &str) -> Option<f64> {
    let re = Regex::new(r"Duration: (\d\d):(\d\d):(\d\d).(\d\d)").unwrap();
    let captures = re.captures(probe_output)?;

    let hours: f64 = captures[1].parse().ok()?;
    let minutes: f64 = captures[2].parse().ok()?;
    let seconds: f64 = captures[3].parse().ok()?;
    let fractional: f64 = captures[4].parse().ok()?;

    Some((hours * 360.) + (minutes * 60.) + seconds + (fractional / 100.))
}

fn pause(code: i32) -> ! {
    print!("Press Enter to continue . . .");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0]).unwrap();
    exit(code)
}
