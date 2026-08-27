
render:
    cargo run --release
    ffmpeg -framerate 24 -i frames/frame%04d.ppm -c:v libx264 -pix_fmt yuv420p output.mp4

clean:
    cargo clean
    rm -f frames/*.ppm output.mp4

