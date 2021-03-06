====
alrm
====

Description
-----------

A WIP alarm program.

Supported platforms:

- Linux
- Windows

Planned platforms:

- Android
- BSD (this actually probably works, but it's untested)

Installation
------------

- ``git clone https://github.com/Phate6660/alrm && cd alrm``
- ``cargo build --release`` OR ``cargo build --release --features=notify``

| I have split the notification off into a feature, so that way the amount
| of dependencies is cut in half for anyone who doesn't even want a notification.

Usage
-----

- ``TIME`` = the time in ``HH:MM`` format (note that it is 24 hr (00-23))
- ``FILE`` = the path to the audio file (flac/wav supported -- ``none`` if not wanted)
- ``MESSAGE`` = the message to be displayed when the alarm goes off

``./target/release/alrm TIME FILE MESSAGE``

| You may also interact with the alarm while it's running 
| by sending commands to ``/tmp/alrm``.
| For example by doing something like ``echo "command" > /tmp/alrm``.
| 
| Commands supported:

- ``status``
- ``stop``

TODO
----

- Allowing queueing of audio
- Status:

 + Current time, alarm set time -- done
 + Remaining time

- Named pipe for commanding the program (e.g. status/stop) -- done
- Network support (e.g. file from server or youtube link)
- Supply some audio clips along with the program, such as:

 + Annoying shrill alarm tones
 + Phone ringing

- Support MP3 and Vorbis (details in comments in source)
