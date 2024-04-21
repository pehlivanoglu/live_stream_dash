use actix_web::{App, HttpServer};
use actix_files as fs;
use actix_cors::Cors;
use tokio::process::Command;
use std::path::Path;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if !Path::new("./video").exists() {
        std::fs::create_dir_all("./video").expect("Failed to create video directory");
    }

    tokio::spawn(async {
        env::set_var("TZ", "Etc/GMT-3");
        let mut child = Command::new("ffmpeg")
            .args([
                "-f", "v4l2",
                "-i", "/dev/video0",
                "-c:v", "libx264",
                "-preset", "ultrafast",
                "-tune", "zerolatency",
                "-g", "30",
                "-keyint_min", "30",
                "-b:v", "3000k",
                "-minrate", "3000k",
                "-maxrate", "3000k",
                "-bufsize", "3000k",
                "-s", "1920x1080",
                "-f", "dash",
                "-vf", "drawtext=text='%{localtime\\:%X}': x=(w-tw)/2: y=h-(2*lh): fontcolor=white: fontsize=24: box=1: boxcolor=black@0.5: boxborderw=5: fontfile='/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf'",
                "-seg_duration", "2",
                "-window_size", "5",
                "-extra_window_size", "5",
                "-use_template", "1",
                "-use_timeline", "1",
                "-streaming", "1",
                "-ldash", "1",
                "-remove_at_exit", "1",
                "./video/manifest.mpd",
            ])
            .spawn()
            .expect("Failed to start ffmpeg process");

        let _result = child.wait().await.expect("Failed to wait on ffmpeg");
    });

    println!("Server started serving!");

    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .supports_credentials()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .service(fs::Files::new("/video", "./video").show_files_listing())
            .service(fs::Files::new("/", "./static").index_file("index.html"))
    })
        .bind("localhost:2323")?
        .run()
        .await

}
