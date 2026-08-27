set shell := ["zsh", "-cu"]

render:
    cargo run --release
    ffmpeg -framerate 12 -i frames/frame%04d.ppm -c:v libx264 -pix_fmt yuv420p output.mp4

clean:
    cargo clean
    rm -f frames/*.ppm output.mp4

flamegraph name:
    mkdir -p flamegraphs; \
    global=$(find flamegraphs -maxdepth 1 -type f -name 'v*-*.svg' | sed -E 's|.*/v([0-9]+)-.*|\1|' | sort -n | tail -1); \
    global=$(( ${global:-0} + 1 )); \
    name_version=$(find flamegraphs -maxdepth 1 -type f -name "*-{{name}}-v*.svg" | sed -E 's|.*-v([0-9]+)\.svg|\1|' | sort -n | tail -1); \
    name_version=$(( ${name_version:-0} + 1 )); \
    output=$(printf "flamegraphs/v%02d-{{name}}-v%02d.svg" "$global" "$name_version"); \
    echo "Creating $output"; \
    cargo flamegraph --release --output "$output"
