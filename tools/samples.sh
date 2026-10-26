#!/bin/sh
# The recordings of tools/samples.txt: `fetch` downloads them from archive.org
# into the artifact store, one folder an item, and decodes each FLAC to a WAV
# beside it with ffmpeg; `copy` brings the WAVs to tmp/samples, which is
# gitignored and what tools/bench reads, and drops any WAV there the store no
# longer has. The store is the one copy kept; tmp is made again from it when
# needed.
set -eu
cd "$(dirname "$0")/.."
store=${SOUND_FLAC_ARTIFACTS:-/mnt/dasdata/backup/sound-artifacts/samples}
case "${1:-}" in
fetch)
    grep -v '^#' tools/samples.txt | while IFS="$(printf '\t')" read -r item file _; do
        mkdir -p "$store/$item"
        if [ ! -s "$store/$item/$file" ]; then
            url="https://archive.org/download/$item/$(python3 -c 'import sys,urllib.parse;print(urllib.parse.quote(sys.argv[1]))' "$file")"
            curl -fL --retry 5 -o "$store/$item/$file.part" "$url" && mv "$store/$item/$file.part" "$store/$item/$file"
        fi
        case "$file" in
        *.flac) wav="$store/$item/${file%.flac}.wav"
            [ -s "$wav" ] || ffmpeg -nostdin -loglevel error -i "$store/$item/$file" -c:a pcm_s$(ffprobe -v error -select_streams a -show_entries stream=bits_per_raw_sample -of csv=p=0 "$store/$item/$file")le "$wav" ;;
        esac
    done
    (cd "$store" && find . \( -name '*.wav' -o -name '*.flac' \) -print0 | sort -z | xargs -0 sha256sum --) > "$store/SHA256SUMS"
    ;;
copy)
    mkdir -p tmp/samples
    rsync -a --delete --include='*/' --include='*.wav' --exclude='*' "$store/" tmp/samples/
    ;;
*)
    echo "usage: tools/samples.sh fetch|copy" >&2
    exit 2
    ;;
esac
