"""The tables of docs/benchmarks.md, from the log of tools/bench:

    cargo run --release --manifest-path tools/bench/Cargo.toml > bench.log
    python3 -I tools/tables.py bench.log

First, over every shape and sound, this encoder's time a block and bytes a
block over libFLAC's at level 5 as v0.0.4 ran it, a table each; then the same
against libFLAC at 0, its fastest, and at 8, its smallest; then each shape
apart, with what each coder takes in microseconds and bytes, the share of one
core a block of 20 ms is, and the share of the PCM it comes to. A ratio is this
encoder over libFLAC; green is this encoder's side of the line, and a ratio of
1.00 is left plain.
"""
import collections
import re
import sys

SHAPES = ["48k-stereo-16", "44.1k-stereo-16", "96k-stereo-16", "48k-mono-16", "48k-stereo-8", "48k-stereo-24"]
KINDS = ["music", "speech", "tone", "noise", "silence"]
FIELD = re.compile(r"(\w+)=(\S+)")


def read(path):
    """(shape, kind) -> coder -> its line's fields, and shape -> (block, pcm)."""
    data, shapes = collections.defaultdict(dict), {}
    for line in open(path):
        f = dict(FIELD.findall(line))
        if "coder" not in f:
            continue
        shapes[f["shape"]] = (int(f["block"]), int(f["pcm"]))
        data[(f["shape"], f["kind"])][f["coder"]] = dict(us=float(f["us"]), bytes=float(f["bytes"]))
    return data, shapes


def colour(text, good):
    if good is None:
        return text
    return "$\\color{%s}{%s}$" % ("green" if good else "red", text)


def ratio(mine, theirs):
    text = f"{mine / theirs:.2f}"
    return colour(text, None if text == "1.00" else mine < theirs)


def shape_name(s):
    rate, channels, bits = s.split("-")
    return f"{rate.replace('k', ' kHz')} {channels} {bits}-bit"


def grid(title, note, data, shapes, field, theirs):
    print(f"### {title}\n\n{note}\n")
    print("| Sound | " + " | ".join(shape_name(s) for s in shapes) + " |")
    print("|---|" + "---:|" * len(shapes))
    for kind in KINDS:
        cells = []
        for s in shapes:
            coders = data.get((s, kind), {})
            cells.append(ratio(coders["sound-flac"][field], coders[theirs][field]) if "sound-flac" in coders and theirs in coders else "—")
        print(f"| {kind} | " + " | ".join(cells) + " |")
    print()


def us(v):
    return f"{v:.1f}" if v < 100 else f"{v:.0f}"


def apart(shape, block, pcm, data):
    print(f"### {shape_name(shape)}, {block} frames a block, {pcm} bytes of PCM\n")
    print("| Sound | libFLAC 5, µs | This encoder, µs | Time ratio | Of one core | libFLAC 5, bytes | This encoder, bytes | Bytes ratio | Of the PCM |")
    print("|---|---:|---:|---:|---:|---:|---:|---:|---:|")
    for kind in KINDS:
        coders = data.get((shape, kind), {})
        if "sound-flac" not in coders or "libflac-5" not in coders:
            continue
        mine, ref = coders["sound-flac"], coders["libflac-5"]
        time = colour(us(mine["us"]), None if us(mine["us"]) == us(ref["us"]) else mine["us"] < ref["us"])
        size = colour(f"{mine['bytes']:.0f}", None if round(mine["bytes"]) == round(ref["bytes"]) else mine["bytes"] < ref["bytes"])
        core = f"{mine['us'] / 20_000 * 100:.2f}%"
        share = f"{mine['bytes'] / pcm * 100:.1f}%"
        print(f"| {kind} | {us(ref['us'])} | {time} | {ratio(mine['us'], ref['us'])} | {core} | {ref['bytes']:.0f} | {size} | {ratio(mine['bytes'], ref['bytes'])} | {share} |")
    print()


def main():
    data, shapes = read(sys.argv[1])
    order = [s for s in SHAPES if s in shapes] + sorted(s for s in shapes if s not in SHAPES)
    print("## Against libFLAC at level 5, as v0.0.4 ran it\n")
    grid("Time a block, this encoder over libFLAC", "Under 1.00 is less of a core than libFLAC at its default level.", data, order, "us", "libflac-5")
    grid("Bytes a block, this encoder over libFLAC", "Under 1.00 is a smaller frame than libFLAC at its default level.", data, order, "bytes", "libflac-5")
    print("## Against libFLAC at its two ends\n")
    grid("Time a block, this encoder over libFLAC at level 0", "Under 1.00 is less of a core than libFLAC's fastest level, which fits no linear predictor and codes stereo as it comes.", data, order, "us", "libflac-0")
    grid("Bytes a block, this encoder over libFLAC at level 8", "Under 1.00 is a smaller frame than libFLAC's smallest level, which tries every order to 12 and every stereo pairing.", data, order, "bytes", "libflac-8")
    print("## Each shape apart\n")
    print("libFLAC is at level 5. The time is one block's, the least of five passes;\na block is 20 ms of sound, so a share of one core is that time over 20 ms.\n")
    for s in order:
        apart(s, *shapes[s], data)


main()
