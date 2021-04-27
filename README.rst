====
alrm
====

Description
-----------

A WIP alarm program that are for sure going to be on Linux and Windows.
Considering Android as well.

Installation
------------

- ``git clone https://github.com/Phate6660/alrm && cd alrm``
- ``cargo build --release`` OR ``cargo build --release --features=notify``

I have split the notification off into a feature, so that way the amount
of dependencies is cut in half for anyone who doesn't even want a notification.

Usage
-----

- ``TIME`` = the amount of time in seconds for the alarm to wait
- ``FILE`` = the path to the audio file (flac/wav supported)
- ``MESSAGE`` = the message to be displayed when the alarm goes off
- ``*`` = this can be anything, but it tells the program to send a notification with the alarm message instead of printing to stdout

``./target/release/alrm TIME FILE MESSAGE *``

TODO
----

- Allowing queueing of audio
- Status:

 + Current time
 + Mode
 + Remaining time

- Multiple Modes:

 + Alarm (set the actual time -- e.g. 5 AM)
 + Stopwatch (set a time -- e.g. 30m)

- Network support (e.g. file from server or youtube link)
- REPL for commanding the program (e.g. pause/start/status/stop)
- Supply some audio clips along with the program, such as:

 + Annoying shrill alarm tones
 + Phone ringing

- Support MP3 and Vorbis (details in comments in source)
