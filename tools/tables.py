"""The tables of docs/benchmarks.md, from the log of tools/bench:

    cargo run --release --manifest-path tools/bench/Cargo.toml > bench.log
    python3 -I tools/tables.py bench.log [bench-arm64.log] [--before before.log]

First, over every shape and made sound, this encoder's time a block and bytes
a block over libFLAC's at level 5 as v0.0.4 ran it, a table each; then the
same against libFLAC at 0, its fastest, and at 8, its smallest; then each
shape apart, with what each coder takes in microseconds and bytes, the share
of one core a block of 20 ms is, and the share of the PCM it comes to. Then
the recordings of tools/samples.txt: every ratio against the three levels in
one table, the block times apart with the 99th-percentile and worst blocks,
and the bytes apart, and, given the log of the run before as `--before`,
this encoder's mean block and bytes in that run beside this one's and this
one over it. Then, given a second log, the same run on an arm64
machine beside the first: this encoder's time over libFLAC's on each, the
made sounds in grids and the desktop's shape and the recordings machine by
machine, and whether the bytes are the same on both. A ratio is this encoder
over libFLAC; green is this encoder's side of the line, and a ratio of 1.00
is left plain.
"""
import collections
import re
import sys

SHAPES = ["48k-stereo-16", "44.1k-stereo-16", "96k-stereo-16", "48k-mono-16", "48k-stereo-8", "48k-stereo-24"]
KINDS = ["music", "speech", "tone", "noise", "silence"]
FIELD = re.compile(r"(\w+)=(\S+)")


RECORDINGS = [
    ("Aria", "Bach, Goldberg Aria, piano"),
    ("Fantasy", "Chopin, Fantasy Op. 49, piano"),
    ("dso2006", "Shostakovich 5, orchestra, live"),
    ("Golf_Club", "Indie rock, 2022, mastered"),
    ("Track10", "Mix at −13 LUFS, wide stereo"),
    ("Track_1", "Mix at −9 LUFS, loud master"),
    ("Limehouse", "Limehouse Blues, 1924 78 rpm"),
    ("JumpinJudy", "Prison work song, 1947 field recording"),
    ("Raven", "The Raven, read, produced"),
    ("Leviticus", "Leviticus 13, read"),
    ("JFK", "JFK at Rice, 1962"),
    ("PotniskiVlak", "Train station, field recording"),
    ("PinkNoise", "Pink noise"),
    ("WhiteNoise", "White noise"),
]


def read(path):
    """(shape, kind) -> coder -> its line's fields, and shape -> (block, pcm),
    for the made sounds; and file -> (shape, seconds, coder -> fields) for the
    recordings, in the order of RECORDINGS."""
    data, shapes, files = collections.defaultdict(dict), {}, {}
    for line in open(path):
        f = dict(FIELD.findall(line))
        if "coder" not in f:
            continue
        numbers = {k: float(f[k]) for k in ("us", "p99", "worst", "bytes") if k in f}
        if "file" in f:
            files.setdefault(f["file"], (f["shape"], int(f["pcm"]), int(f.get("seconds", 0)), {}))[3][f["coder"]] = numbers
        else:
            shapes[f["shape"]] = (int(f["block"]), int(f["pcm"]))
            data[(f["shape"], f["kind"])][f["coder"]] = numbers
    order = sorted(files, key=lambda name: next((i for i, (key, _) in enumerate(RECORDINGS) if key in name.rsplit("/", 1)[-1]), len(RECORDINGS)))
    return data, shapes, [(name, *files[name]) for name in order]


def recording_name(file):
    base = file.rsplit("/", 1)[-1]
    return next((label for key, label in RECORDINGS if key in base), f"`{file}`")


def ratio3(mine, theirs):
    text = f"{mine / theirs:.3f}"
    return colour(text, None if text == "1.000" else mine < theirs)


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


def recordings(files, before):
    """The recordings' tables; `before` is file -> this encoder's fields in the
    run before, or None, when the block times and the bytes carry that run
    beside this one."""
    with_all = [(name, shape, pcm, seconds, c) for name, shape, pcm, seconds, c in files if all(k in c for k in ("sound-flac", "libflac-5", "libflac-0", "libflac-8"))]
    if not with_all:
        return

    def earlier(name, field, now, fmt):
        """The run before's cell and this run over it, or two blanks."""
        if before is None:
            return []
        if name not in before or field not in before[name]:
            return ["", ""]
        then = before[name][field]
        return [fmt(then), colour(f"{now / then:.2f}", None if f"{now / then:.2f}" == "1.00" else now < then)]
    print("## The recordings\n")
    print("The recordings of `tools/samples.txt`, from archive.org, each coded\nwhole in blocks of 20 ms, the time a block the least of three passes' mean.\n")
    print("### Against libFLAC at levels 5, 0 and 8\n")
    print("This encoder over libFLAC: the time a block and the bytes a block, at each of\nthe three levels.\n")
    print("| Recording | Shape | Time, 5 | Bytes, 5 | Time, 0 | Bytes, 0 | Time, 8 | Bytes, 8 |")
    print("|---|---|---:|---:|---:|---:|---:|---:|")
    for name, shape, pcm, seconds, c in with_all:
        mine = c["sound-flac"]
        cells = [ratio(mine[field], c[level][field]) for level in ("libflac-5", "libflac-0", "libflac-8") for field in ("us", "bytes")]
        print(f"| {recording_name(name)} | {shape_name(shape)} | " + " | ".join(cells) + " |")
    print()
    print("### The block times, libFLAC at level 5\n")
    print("A block is 20 ms of sound. The 99th-percentile and worst blocks are the\nleast over the passes, so a pass the machine disturbed does not set them;\n*of one core* is the mean block over 20 ms." + (" *The run before* is this\nencoder's mean block in the run of the commit before this one, and *over it*\nthis run's over that." if before is not None else "") + "\n")
    extra = " The run before, mean µs | Over it |" if before is not None else ""
    print("| Recording | libFLAC, mean µs | This encoder, mean µs | Ratio |" + extra + " libFLAC, 99th | This encoder, 99th | libFLAC, worst | This encoder, worst | Of one core |")
    print("|---|---:|---:|---:|" + ("---:|---:|" if before is not None else "") + "---:|---:|---:|---:|---:|")
    for name, shape, pcm, seconds, c in with_all:
        mine, ref = c["sound-flac"], c["libflac-5"]
        cells = []
        for field in ("us", "p99", "worst"):
            cells.append(us(ref[field]))
            cells.append(colour(us(mine[field]), None if us(mine[field]) == us(ref[field]) else mine[field] < ref[field]))
            if field == "us":
                cells.append(ratio(mine["us"], ref["us"]))
                cells.extend(earlier(name, "us", mine["us"], us))
        print(f"| {recording_name(name)} | " + " | ".join(cells) + f" | {mine['us'] / 20_000 * 100:.2f}% |")
    print()
    print("### The bytes, libFLAC at level 5\n")
    if before is not None:
        print("*The run before* is this encoder's bytes a block in the run of the commit\nbefore this one, and *over it* this run's over that, to a ten-thousandth.\n")
    extra = " The run before, bytes | Over it |" if before is not None else ""
    print("| Recording | Seconds | PCM a block | libFLAC, bytes | This encoder, bytes | Ratio |" + extra + " Of the PCM |")
    print("|---|---:|---:|---:|---:|---:|" + ("---:|---:|" if before is not None else "") + "---:|")
    for name, shape, pcm, seconds, c in with_all:
        mine, ref = c["sound-flac"], c["libflac-5"]
        size = colour(f"{mine['bytes']:.0f}", None if round(mine["bytes"]) == round(ref["bytes"]) else mine["bytes"] < ref["bytes"])
        cells = []
        if before is not None:
            if name in before and "bytes" in before[name]:
                text = f"{mine['bytes'] / before[name]['bytes']:.4f}"
                cells = [f"{before[name]['bytes']:.0f}", colour(text, None if text == "1.0000" else mine["bytes"] < before[name]["bytes"])]
            else:
                cells = ["", ""]
        print(f"| {recording_name(name)} | {seconds} | {pcm} | {ref['bytes']:.0f} | {size} | {ratio3(mine['bytes'], ref['bytes'])} | " + "".join(f"{cell} | " for cell in cells) + f"{mine['bytes'] / pcm * 100:.1f}% |")
    print()


def arm64(data, files, data2, files2, order):
    """The second log's run beside the first's, machine by machine."""
    print("## On arm64\n")
    print("The same run on an arm64 machine, beside the x86 run above: this encoder's\ntime a block over libFLAC's on each. A ratio under 1.00 is less of a core\nthan libFLAC on that machine, and a block's µs are that machine's.\n")
    grid("Time a block on arm64, this encoder over libFLAC at level 5", "Under 1.00 is less of a core than libFLAC at its default level, on arm64.", data2, order, "us", "libflac-5")
    grid("Time a block on arm64, this encoder over libFLAC at level 0", "Under 1.00 is less of a core than libFLAC's fastest level, on arm64.", data2, order, "us", "libflac-0")
    desktop = "48k-stereo-16"
    if desktop in order:
        print(f"### The made sounds at {shape_name(desktop)}, machine by machine\n")
        print("libFLAC is at level 5; the time is one block's, the least of five passes.\n")
        print("| Sound | libFLAC, x86 µs | libFLAC, arm64 µs | This encoder, x86 µs | This encoder, arm64 µs | Ratio, x86 | Ratio, arm64 |")
        print("|---|---:|---:|---:|---:|---:|---:|")
        for kind in KINDS:
            a, b = data.get((desktop, kind), {}), data2.get((desktop, kind), {})
            if not all(k in c for c in (a, b) for k in ("sound-flac", "libflac-5")):
                continue
            print(f"| {kind} | {us(a['libflac-5']['us'])} | {us(b['libflac-5']['us'])} | {us(a['sound-flac']['us'])} | {us(b['sound-flac']['us'])} | {ratio(a['sound-flac']['us'], a['libflac-5']['us'])} | {ratio(b['sound-flac']['us'], b['libflac-5']['us'])} |")
        print()
    theirs = {name: c for name, _, _, _, c in files2}
    both = [(name, c, theirs[name]) for name, _, _, _, c in files if name in theirs]
    both = [(name, a, b) for name, a, b in both if all(k in c for c in (a, b) for k in ("sound-flac", "libflac-5", "libflac-0"))]
    if both:
        print("### The recordings, machine by machine\n")
        print("libFLAC is at level 5 but in the last two columns; the time is the mean\nblock, and the 99th-percentile block is this encoder's, the least over the passes.\n")
        print("| Recording | libFLAC, x86 µs | libFLAC, arm64 µs | This encoder, x86 µs | This encoder, arm64 µs | Ratio, x86 | Ratio, arm64 | 99th, x86 | 99th, arm64 | Time vs 0, x86 | Time vs 0, arm64 |")
        print("|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|")
        for name, a, b in both:
            cells = [us(a["libflac-5"]["us"]), us(b["libflac-5"]["us"]), us(a["sound-flac"]["us"]), us(b["sound-flac"]["us"]),
                     ratio(a["sound-flac"]["us"], a["libflac-5"]["us"]), ratio(b["sound-flac"]["us"], b["libflac-5"]["us"]),
                     us(a["sound-flac"]["p99"]), us(b["sound-flac"]["p99"]),
                     ratio(a["sound-flac"]["us"], a["libflac-0"]["us"]), ratio(b["sound-flac"]["us"], b["libflac-0"]["us"])]
            print(f"| {recording_name(name)} | " + " | ".join(cells) + " |")
        print()
    coders = {"sound-flac": "this encoder", "libflac-5": "libFLAC at level 5", "libflac-0": "libFLAC at level 0", "libflac-8": "libFLAC at level 8"}
    cells = [(f"{kind} at {shape_name(shape)}", c, data2[(shape, kind)]) for (shape, kind), c in data.items() if (shape, kind) in data2]
    cells += [(recording_name(name), a, b) for name, a, b in both]
    for coder, who in coders.items():
        differ = [f"{what} ({a[coder]['bytes']:.0f} against {b[coder]['bytes']:.0f})" for what, a, b in cells if coder in a and coder in b and abs(a[coder]["bytes"] - b[coder]["bytes"]) >= 0.05]
        if differ:
            print(f"The bytes a block of {who} differ between the two machines on " + ", ".join(differ) + ".\n")
        else:
            print(f"The bytes a block of {who} are the same on both machines, to a tenth of a byte, on every made sound and every recording.\n")


def main():
    args = sys.argv[1:]
    before = None
    if "--before" in args:
        at = args.index("--before")
        _, _, earlier = read(args[at + 1])
        before = {name: c["sound-flac"] for name, _, _, _, c in earlier if "sound-flac" in c}
        del args[at:at + 2]
    data, shapes, files = read(args[0])
    order = [s for s in SHAPES if s in shapes] + sorted(s for s in shapes if s not in SHAPES)
    print("## The made sounds, against libFLAC at level 5 as v0.0.4 ran it\n")
    grid("Time a block, this encoder over libFLAC", "Under 1.00 is less of a core than libFLAC at its default level.", data, order, "us", "libflac-5")
    grid("Bytes a block, this encoder over libFLAC", "Under 1.00 is a smaller frame than libFLAC at its default level.", data, order, "bytes", "libflac-5")
    print("## The made sounds, against libFLAC at its two ends\n")
    grid("Time a block, this encoder over libFLAC at level 0", "Under 1.00 is less of a core than libFLAC's fastest level, which fits no linear predictor and codes stereo as it comes.", data, order, "us", "libflac-0")
    grid("Bytes a block, this encoder over libFLAC at level 8", "Under 1.00 is a smaller frame than libFLAC's smallest level, which tries every order to 12 and every stereo pairing.", data, order, "bytes", "libflac-8")
    print("## The made sounds, each shape apart\n")
    print("libFLAC is at level 5. The time is one block's, the least of five passes;\na block is 20 ms of sound, so a share of one core is that time over 20 ms.\n")
    for s in order:
        apart(s, *shapes[s], data)
    recordings(files, before)
    if len(args) > 1:
        data2, _, files2 = read(args[1])
        arm64(data, files, data2, files2, order)


main()
