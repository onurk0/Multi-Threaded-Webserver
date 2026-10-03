# Multi-Threaded Webserver

A small Rust web server that serves static files from a dedicated `public` folder and redirects requests on port 7777 to Google.

## Features

- Serves HTML, images, audio, and video files from `public/`
- Supports multi-threaded request handling on port 8888
- Returns a 301 redirect from port 7777 to `http://www.google.com`
- Shows a simple 404 page when a requested file is missing

## Prerequisites

- Rust and Cargo installed
- A browser to view the site locally

## Run the project

1. Open a terminal in the project root.
2. Build the project:

   ```bash
   cargo build
   ```

3. Start the server:

   ```bash
   cargo run
   ```

4. Open one of these URLs in your browser:

   - `http://localhost:8888/`
   - `http://127.0.0.1:8888/`

## Behavior

- Port `8888` serves the files in `public/`
- Port `7777` immediately redirects to Google
- Requests for missing files show the 404 page from `public/404.html`

## Project layout

```text
Multi-Threaded-Webserver/
├── Cargo.toml
├── src/
│   └── main.rs
├── public/
│   ├── index.html
│   ├── 404.html
│   ├── *.jpg
│   ├── *.mp3
│   ├── *.mp4
│   └── *.webm
├── target/
└── README.md
```

## Notes

If you want to stop the server, press `Ctrl + C` in the terminal where it is running.
