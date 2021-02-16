# Autosplitter
Autosplitter is a simple utility to automatically split audio files for use in Celeste's cassette rooms.

Some antivirus programs, including Avast, may block ffmpeg, preventing autosplitter from running properly.
If you have an antivirus program installed please add an exemption to the file `%TEMP%\ffmpeg.exe`.

Autosplitter comes with a plugin for FMOD to bulk add assets to a cassette track, which you can use on the split files once you've ran this on your source audio and imported the files into FMOD.
Drop the provided bulkadd.js file into `%localappdata%\FMOD Studio\Scripts\` and run Scripts->Bulk Add in FMOD with all of the sections and track you'd like to add to selected.

## Building from source
To build, you will need to provide your own ffmpeg.exe in the source directory; you can download a copy from https://ffmpeg.org/.
