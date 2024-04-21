# Live Stream Video Surveillance using MPEG-DASH

## Introduction
This project enables streaming live video using Dynamic Adaptive Streaming over HTTP (DASH) from a camera connected to a server. Built with Rust and ffmpeg, it features a web interface accessible via browsers for real-time surveillance monitoring.

## Features
- **Client**: Utilizes HTML5, DASH.js, CSS, and JavaScript for a dynamic viewing experience with controls for video playback.
- **Server**: Rust-based server setup with ffmpeg for video capture and encoding. Integrated with Actix-web and Ngrok to facilitate external access and manage CORS for cross-origin requests.

## Installation and Usage

### Prerequisites
- Rust and Cargo installed. Verify with `rustc --version` and `cargo --version`.
- Ngrok installed for external access. Verify installation with `ngrok --version`.
- Ffmpeg for encoding. Verify installation with `ffmpeg --version`.
### Setup
1. **Clone and Navigate**:
```
git clone https://github.com/pehlivanoglu/live_stream_dash
cd live_stream_dash
```

2. **Build and Run**:
```
cargo run
```

### Running Without Rust and Cargo
1. **Clone and Navigate**:
```
git clone https://github.com/pehlivanoglu/live_stream_dash
cd live_stream_dash
```
2. **Run**:
```
./target/release/video_surveillance
```


### Exposing Server with Ngrok
- To expose the local server and allow external access:
```
ngrok http 2323
```
Replace `2323` with the actual port number where your server listens. This command provides a URL that can be used to access the server from the internet.

Streams video to `/video` directory with DASH chunks and `manifest.mpd`.

### Access
- Open your browser and go to the provided Ngrok URL or the server's local IP address to view and control the live stream.

## Notes
- Keep file and folder locations intact to avoid path issues in `/src/main.rs` and `/static/scripts.js`. If changes are made, adjust paths and rebuild.
