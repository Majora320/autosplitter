This is a simple utility written for Windows to automatically split audio files for use in Celeste's cassette rooms. 
To build, you will need to provide your own ffmpeg.exe in the source directory; you can download a copy from https://ffmpeg.org/.
Some antivirus programs, including Avast, may block ffmpeg.
If you have an antivirus program installed please add an exemption to the file `%TEMP%\ffmpeg.exe`, or this program will fail.
This also comes with a plugin for FMOD to bulk import assets to a cassette track.
Drop the provided bulkadd.js file into `%localappdata%\FMOD Studio\Scripts\` and run Scripts->Bulk Add in FMOD with the assets and track selected.