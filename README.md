# bebops

Simple, all-in-one music downloader and tagger.

This tool works by pulling information about videos using `yt-dlp`.
Then edit the data to your liking and download. The videos are
converted to `mp3` and metadata applied automatically.

## Dependencies

- [yt-dlp](https://github.com/yt-dlp/yt-dlp#installation)
- [ffmpeg](https://www.ffmpeg.org/download.html)

Both need to be in the $PATH environment variable.

## Installation

For now, the Rust toolchain is needed in order to compile and run.

Clone the repo, then run `cargo run` to compile and run.

Installers and/or packages are planned for multiple platforms.

## Usage

Basic flow:
- Pick a download type from the dropdown
- Paste YT link and click fetch
- Edit metadata
- Click download

The fetching can take a second, and the downloading a bit longer.
Playlists take slighlty longer to fetch because each video has to be fetched.
It may appear to hang, just give it a few seconds.

Downloads are of five main types:
- Singles
- Playlists
- Albums
- Full-Video Playlists
- Full-Video Albums

When fetching information with `yt-dlp`, if metadata information is already present,
it is automatically applied. The rest is up to you to add/tweak to your liking.

Playlists/albums are given as YT playlists.
FV playlists/Albums are given as single videos to be split into multiple songs.

For cover art, video thumbnails are used by default, but alternative images can be given.

Downloads go to your `Music` directory, whichever that is on your platform,
in the `bebops` folder.

### Single

Stupid easy, give a title, artist, album/thumbnail image if you feel like it.

### Playlists/Albums

Give your YT playlist link and wait for fetch to finish. Edit the playlist title,
you can also choose to apply a single cover to each song. Each song's data
can be tweaked individually.

Top level fields like album title/artist, and cover art can be specified
and easily be applied to each song.

For playlists, bare bones `m3u8` playlist file will also be
generated in the playlist's download folder.

### Full-Videos

Single videos can be given and split into individual songs.
Real difference to playlist links is how chapters are specified.

If the video has chapters, they are applied to the data, or you manually add and remove chapters.
Each chapter has a start and end timestamp. You can drag the box or type a time in the given format.

## Notes

Compared to other music taggers this is a very minimal tool. I wrote this because
every once in a while, I just want to pull a video and tag the essentials, nothing more.

This project should be useable without issues, but technically early in the dev refinement process.
