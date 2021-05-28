#[cfg(feature = "notify")]
use notify_rust::{Notification, Timeout};

use chrono::Timelike;
use rodio::{Decoder, OutputStream, source::Source};
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn help() {
    println!("3 args need to be passed to this program.\n\
             The time in `HH:MM` format (note that it 24 (00-23) hour),\n\
             the path to the the audio file (none if no audio wanted), and the alarm message.");
    std::process::exit(0);
}

#[cfg(feature = "notify")]
fn notify(message: &str) {
    Notification::new()
        .summary(&message)
        .timeout(Timeout::Milliseconds(6000))
        .show()
        .unwrap();
}

fn alarm(msg: &String) {
    // Display the alarm message.
    // Print to stdout if not using the notification feature.
    #[cfg(not(feature = "notify"))]
    println!("{}", msg);

    // Send a notification of the notification feature is enabled.
    #[cfg(feature = "notify")]
    notify(&msg);
}

fn get_time() -> String {
    let local = chrono::Local::now();
    format!("{:02}:{:02}", local.hour(), local.minute())
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    // If the 2nd element in the argument vector doesn't exist
    // (meaning no args were passed), then display the help message.
    if args.get(1) == None { help(); }

    let time = &args[1];
    // Match the first arg to the program for common help arguments and display the help message.
    match time.as_str() {
        "h" => help(),
        "-h" => help(),
        "help" => help(),
        "--help" => help(),
        _ => (),
    };

    let path = &args[2];
    let msg = &args[3];

    let mut localtime = get_time();
    println!("Alarm set to {}, it is currently {}.", time, localtime);

    // The file to be used as a way to input commands.
    let pipe_path = "/tmp/alrm";
    // Create the file by writing nothing to it.
    std::fs::write(pipe_path, "")?;
    loop {
        // Open the file to view inputted commands.
        let pipe = File::open(pipe_path)?;
        let mut buf_reader = BufReader::new(pipe);
        let mut command = String::new();

        // Store the contents of the file into the command variable.
        buf_reader.read_to_string(&mut command)?;

        // Overwrite the file to be blank again so that the command isn't ran more than once.
        std::fs::write(pipe_path, "")?;

        match command.trim() {
            "status" => {
                let status_message = 
                    format!("Current time:   {}\nAlarm set time: {}", localtime, time);

                #[cfg(not(feature = "notify"))]
                println!("{}", status_message);

                #[cfg(feature = "notify")]
                notify(&status_message);
            },
            "stop" => {
                let stop_message = "Stopped due to user command!";

                #[cfg(not(feature = "notify"))]
                println!("{}", stop_message);

                #[cfg(feature = "notify")]
                notify(&stop_message);

                std::process::exit(0);
            },
            _ => ()
        }

        // Wait 1 second.
        std::thread::sleep(std::time::Duration::from_secs(1));

        // If the count of loops is equal to time, break and start playing audio,
        // Else up the value of count and continue.
        if &localtime == time {
            break;
        } else {
            localtime = get_time();
            continue;
        }
    }

    if path != "none" {
        // TODO: Perhaps allow for the queueing of songs?
        // TODO: Network support (e.g. inputting a youtube url).
        // TODO: Supply some audio clips with the program, such as a phone ringing or a shrill alarm clock noise.
        // Get a output stream handle to the default physical sound device.
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        // Load the audio file.
        let file = BufReader::new(File::open(path).unwrap());
        // Decode that sound file into a source.
        let source = Decoder::new(file).unwrap();
        // Get the duration of the song, this is the reason why only FLAC and WAV are supported,
        // because the rodio lib only supports getting the duration from them.
        // So... TODO: manually add in support for MP3 (I found a lib for that so it shouldn't take to
        // long) as well as Vorbis.
        let duration = source.total_duration().unwrap();
        // Play the sound directly on the device.
        stream_handle.play_raw(source.convert_samples()).unwrap();
        alarm(msg);
        std::thread::sleep(duration);
    } else {
        alarm(msg);
    }
    Ok(())
}
