#[cfg(feature = "notify")]
use notify_rust::{Notification, Timeout};

use rodio::{Decoder, OutputStream, source::Source};
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn help() {
    println!("3 args need to be passed to this program.\n\
             The time to wait, the path to the the audio file, and the alarm message.");
    std::process::exit(0);
}

#[cfg(feature = "notify")]
fn display(msg: &str) {
    Notification::new()
        .summary(&msg)
        .timeout(Timeout::Milliseconds(6000)) // Notification closes in 6s
        .show()
        .unwrap();
}

#[cfg(not(feature = "notify"))]
fn display(msg: &str) {
    println!("{}", msg);
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

    let time = time.parse::<u64>().unwrap();
    println!("Alarm set for {} seconds.", time);

    // The file to be used as a way to input commands.
    let pipe_path = "/tmp/alrm";
    // Create the file by writing nothing to it.
    std::fs::write(pipe_path, "")?;
    let mut count = 0;
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
                let status_message = format!("Seconds Passed: {}\nAmount Set:     {}", count, time);

                #[cfg(not(feature = "notify"))]
                println!("{}", status_message);

                #[cfg(feature = "notify")]
                Notification::new()
                    .summary(&status_message)
                    .timeout(Timeout::Milliseconds(6000))
                    .show()
                    .unwrap();
            },
            "stop" => {
                let stop_message = "Stopped due to user command!";

                #[cfg(not(feature = "notify"))]
                println!("{}", stop_message);

                #[cfg(feature = "notify")]
                Notification::new()
                    .summary(&stop_message)
                    .timeout(Timeout::Milliseconds(6000))
                    .show()
                    .unwrap();

                std::process::exit(0);
            },
            _ => ()
        }

        // Wait 1 second.
        std::thread::sleep(std::time::Duration::from_secs(1));

        // If the count of loops is equal to time, break and start playing audio,
        // Else up the value of count and continue.
        if count == time {
            break;
        } else {
            count += 1;
            continue;
        }
    }

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

    // Display the alarm message.
    display(msg);

    // Wait for the audio to stop before quitting the program.
    std::thread::sleep(duration);
    Ok(())
}
