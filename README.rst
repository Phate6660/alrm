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
- ``cargo build --release``

Usage
-----

- ``TIME`` = the amount of time in seconds for the alarm to wait
- ``FILE`` = the path to the audio file (flac/mp3/vorbis/wav supported)
- ``MESSAGE`` = the message to be displayed when the alarm goes off

``./target/release/alrm TIME FILE MESSAGE``

TODO
----

- Allowing queueing of audio
- Network support (e.g. file from server or youtube link)
- Supply some audio clips along with the program, such as:
 + Annoying shrill alarm tones
 + Phone ringing
