use notify_rust::{Notification, Timeout};
use rusty_audio::Audio;

fn help() {
    println!("3 args need to be passed to this program.\n\
             The time to wait, the path to the the audio file, and the alarm message.");
    std::process::exit(0);
}

fn main() {
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

    let file = &args[2];
    let msg = &args[3];

    let time = time.parse::<u64>().unwrap();
    println!("Alarm set for {} seconds.", time);
    std::thread::sleep(std::time::Duration::from_secs(time));

    // Create a new audio soundsystem
    let mut audio = Audio::new();

    // Add the song to play, then play it.
    // TODO: Perhaps allow for the queueing of songs?
    // TODO: Network support (e.g. inputting a youtube url).
    // TODO: Supply some audio clips with the program, such as a phone ringing or a shrill alarm clock noise.
    audio.add("alarm song", file);
    audio.play("alarm song");

    // Display the alarm message.
    if args.get(4).is_some() {
        Notification::new()
            .summary(&msg)
            .timeout(Timeout::Milliseconds(6000)) // Notification closes in 6s
            .show()
            .unwrap();
    } else {
        println!("{}", msg);
    }

    // Wait for the audio to stop before quitting the program.
    audio.wait();
}
