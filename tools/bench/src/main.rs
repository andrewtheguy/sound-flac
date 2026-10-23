//! The crate against libFLAC on 20 ms blocks of a desktop's sound, a line a
//! cell for `tools/tables.py` to make the tables of `docs/benchmarks.md` from:
//!
//!     cargo run --release --manifest-path tools/bench/Cargo.toml > bench.log
//!     python3 -I tools/tables.py bench.log
//!
//! Every shape is coded by libFLAC at its default level 5 as v0.0.4 of this
//! crate ran it, a stream of its own for every frame; by libFLAC at 0, its
//! fastest, and 8, its smallest, the same way; and by this crate. A cell's
//! time is the least of five passes' mean block over fifty blocks, each block
//! timed on its own, after one pass to warm up; its bytes are the frames'
//! over the fifty.
//! Then every WAV under `tmp/samples` (or `SAMPLES`), the recordings of
//! `tools/samples.txt` that `tools/samples.sh copy` brings there, is coded
//! whole in blocks of 20 ms by each coder, three passes after one to warm up,
//! each pass from an encoder that has seen no block, so that what an
//! encoder carries from block to block falls on the same blocks in every
//! pass: a recording's time is the least pass's mean block, and with it the
//! least of the passes' 99th-percentile and worst blocks, the latency a
//! stream sees, and `worst_at`, the block that was slowest at its least over
//! the passes, which is the encoder's doing where a pass's worst may be the
//! machine's. Arguments narrow the shapes by substring (`48k`, `mono`);
//! `KINDS` the made sounds (`KINDS=music,tone`, `KINDS=` for none); `FILES`
//! the recordings the same way; `coder=` the coders
//! (`coder=sound-flac,libflac-5`). `BLOCKS=dir` writes each measurement's
//! blocks to a file there, a block's least time and its bytes a line.
use std::ffi::c_void;
use std::path::{Path, PathBuf};
use std::ptr;
use std::time::Instant;

#[derive(Clone, Copy)]
struct Shape {
    rate: u32,
    channels: u8,
    bits: u8,
}

impl Shape {
    fn block(&self) -> usize {
        (self.rate / 50) as usize
    }
    fn samples(&self) -> usize {
        self.block() * self.channels as usize
    }
    fn pcm_bytes(&self) -> usize {
        self.samples() * self.bits.div_ceil(8) as usize
    }
    fn name(&self) -> String {
        let rate = if self.rate.is_multiple_of(1000) { format!("{}k", self.rate / 1000) } else { format!("{:.1}k", self.rate as f64 / 1000.0) };
        let channels = match self.channels {
            1 => "mono",
            2 => "stereo",
            _ => unreachable!(),
        };
        format!("{rate}-{channels}-{}", self.bits)
    }
}

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> f64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        (self.0 >> 11) as f64 / (1u64 << 53) as f64 - 0.5
    }
}

const KINDS: [&str; 5] = ["music", "speech", "tone", "noise", "silence"];

/// `blocks` blocks of one kind of sound, interleaved, at the shape's width.
fn signal(shape: Shape, blocks: usize, kind: &str) -> Vec<i32> {
    use std::f64::consts::TAU;
    let mut rng = Rng(0x9E37_79B9_7F4A_7C15);
    let n = blocks * shape.block();
    let full = ((1i64 << (shape.bits - 1)) - 1) as f64;
    let mut out = Vec::with_capacity(n * shape.channels as usize);
    for i in 0..n {
        let t = i as f64 / shape.rate as f64;
        let (l, r) = match kind {
            "silence" => (0.0, 0.0),
            // A chord with harmonics and slow vibrato, a bass note, a little
            // room noise, and a right channel that is the left mixed with
            // another note: music.
            "music" => {
                let vib = 1.0 + 0.003 * (t * 5.5 * TAU).sin();
                let mut l = 0.0;
                for (k, f) in [220.0, 277.18, 329.63].iter().enumerate() {
                    for h in 1..=5 {
                        l += (0.3 / (h as f64 * (k + 1) as f64)) * (t * f * vib * h as f64 * TAU).sin();
                    }
                }
                let bass = 0.2 * (t * 55.0 * TAU).sin();
                let r = 0.8 * l + 0.15 * (t * 440.0 * TAU).sin() + bass;
                (l + bass + 0.002 * rng.next(), r + 0.002 * rng.next())
            }
            // One voice-like tone, its pitch and loudness moving, with breath
            // noise, the same on both channels.
            "speech" => {
                let f0 = 120.0 + 30.0 * (t * 2.0 * TAU).sin();
                let mut v = 0.0;
                for h in 1..=12 {
                    v += (0.25 / h as f64) * (t * f0 * h as f64 * TAU + 0.3 * h as f64).sin();
                }
                let env = 0.5 + 0.5 * (t * 3.0 * TAU).sin();
                let v = v * env + 0.01 * rng.next();
                (v, v)
            }
            // A pure tone at -6 dBFS, a notification's beep: a resonance, the
            // ill-conditioned fit.
            "tone" => {
                let v = 0.5 * (t * 1000.0 * TAU).sin();
                (v, 0.7 * v)
            }
            // Uniform noise peaking at -26 dBFS, a twentieth of full scale:
            // the worst a desktop sends, nothing to predict.
            "noise" => (0.1 * rng.next(), 0.1 * rng.next()),
            _ => unreachable!(),
        };
        for c in 0..shape.channels {
            let v = if c % 2 == 0 { l } else { r };
            out.push((v.clamp(-1.0, 1.0) * full).round() as i32);
        }
    }
    out
}

trait Coder {
    fn encode(&mut self, block: &[i32], out: &mut Vec<u8>);
    /// Back to the state before any block, at the start of a pass: an
    /// encoder that carries something from block to block then costs each
    /// block the same in every pass, and the least over the passes is the
    /// block's own.
    fn restart(&mut self) {}
}

/// libFLAC as v0.0.4 ran it: a stream of its own per frame, level 5.
struct RefCoder(libflac_ref::Encoder);
impl Coder for RefCoder {
    fn encode(&mut self, block: &[i32], out: &mut Vec<u8>) {
        self.0.encode(block, out).unwrap();
    }
}

/// libFLAC at a chosen level, still a stream of its own per frame.
struct LibFlac {
    shape: Shape,
    level: u32,
}
unsafe extern "C" fn keep(_e: *const flac_sys::FLAC__StreamEncoder, buffer: *const u8, bytes: usize, samples: u32, _f: u32, client: *mut c_void) -> flac_sys::FLAC__StreamEncoderWriteStatus {
    if samples > 0 {
        unsafe { (*client.cast::<Vec<u8>>()).extend_from_slice(std::slice::from_raw_parts(buffer, bytes)) };
    }
    flac_sys::FLAC__StreamEncoderWriteStatus_FLAC__STREAM_ENCODER_WRITE_STATUS_OK
}
impl Coder for LibFlac {
    fn encode(&mut self, block: &[i32], out: &mut Vec<u8>) {
        unsafe {
            let e = flac_sys::FLAC__stream_encoder_new();
            flac_sys::FLAC__stream_encoder_set_streamable_subset(e, 0);
            flac_sys::FLAC__stream_encoder_set_do_md5(e, 0);
            flac_sys::FLAC__stream_encoder_set_channels(e, self.shape.channels as u32);
            flac_sys::FLAC__stream_encoder_set_bits_per_sample(e, self.shape.bits as u32);
            flac_sys::FLAC__stream_encoder_set_sample_rate(e, self.shape.rate);
            flac_sys::FLAC__stream_encoder_set_compression_level(e, self.level);
            flac_sys::FLAC__stream_encoder_set_blocksize(e, self.shape.block() as u32);
            assert_eq!(flac_sys::FLAC__stream_encoder_init_stream(e, Some(keep), None, None, None, ptr::from_mut(out).cast()), 0);
            assert_ne!(flac_sys::FLAC__stream_encoder_process_interleaved(e, block.as_ptr(), self.shape.block() as u32), 0);
            assert_ne!(flac_sys::FLAC__stream_encoder_finish(e), 0);
            flac_sys::FLAC__stream_encoder_delete(e);
        }
    }
}

/// This crate.
struct Own(sound_flac::Encoder);
impl Coder for Own {
    fn encode(&mut self, block: &[i32], out: &mut Vec<u8>) {
        self.0.encode(block, out).unwrap();
    }
    fn restart(&mut self) {
        self.0 = sound_flac::Encoder::new(self.0.stream()).unwrap();
    }
}

/// One coder over the blocks of `samples`: the least of `passes` passes'
/// mean block in microseconds, and the least of their 99th-percentile and
/// worst blocks, each pass timed block by block after one pass to warm up;
/// and the bytes a block.
fn measure(shape: Shape, samples: &[i32], coder: &mut dyn Coder, passes: usize, least_ms: u128, dump: Option<PathBuf>) -> Measured {
    let blocks = samples.len() / shape.samples();
    let mut out = Vec::new();
    let mut times = vec![0f64; blocks];
    // Each block's size, from the warm-up pass, and the least time each block
    // was ever coded in: the block that is slow in every pass is the
    // encoder's doing, where a pass's worst may be the machine's.
    let mut sizes = vec![0usize; blocks];
    for (block, size) in samples.chunks_exact(shape.samples()).zip(sizes.iter_mut()) {
        out.clear();
        coder.encode(block, &mut out);
        *size = out.len();
    }
    let bytes = sizes.iter().sum::<usize>() as f64 / blocks as f64;
    let mut pass = |out: &mut Vec<u8>, times: &mut [f64]| {
        coder.restart();
        for (block, t) in samples.chunks_exact(shape.samples()).zip(times.iter_mut()) {
            // Each block into an empty buffer, as the warm-up coded it: no
            // frame's time holds the growth of a buffer of every frame before.
            out.clear();
            let start = Instant::now();
            coder.encode(block, out);
            *t = start.elapsed().as_secs_f64() * 1e6;
        }
    };
    let mut least = vec![f64::INFINITY; blocks];
    let (mut mean, mut p99, mut worst) = (f64::INFINITY, f64::INFINITY, f64::INFINITY);
    for _ in 0..passes {
        let start = Instant::now();
        let mut done = 0;
        let (mut sum, mut p, mut w) = (0.0, f64::INFINITY, f64::INFINITY);
        // At least `least_ms` of passes, so a short signal is timed long enough.
        while done == 0 || start.elapsed().as_millis() < least_ms {
            pass(&mut out, &mut times);
            sum += times.iter().sum::<f64>();
            for (l, &t) in least.iter_mut().zip(&times) {
                *l = l.min(t);
            }
            let mut sorted = times.clone();
            sorted.sort_by(f64::total_cmp);
            p = p.min(sorted[(blocks * 99 / 100).min(blocks - 1)]);
            w = w.min(sorted[blocks - 1]);
            done += 1;
        }
        mean = mean.min(sum / (done * blocks) as f64);
        p99 = p99.min(p);
        worst = worst.min(w);
    }
    let worst_at = (0..blocks).max_by(|&a, &b| least[a].total_cmp(&least[b])).unwrap_or(0);
    if let Some(dump) = dump {
        let lines: String = least.iter().zip(&sizes).enumerate().map(|(i, (t, s))| format!("{i}\t{t:.2}\t{s}\n")).collect();
        std::fs::write(dump, lines).unwrap();
    }
    Measured { mean, p99, worst, worst_at, bytes }
}

struct Measured {
    mean: f64,
    p99: f64,
    worst: f64,
    /// The block that was slowest at its least over the passes.
    worst_at: usize,
    bytes: f64,
}

impl std::fmt::Display for Measured {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "us={:.2} p99={:.2} worst={:.2} worst_at={} bytes={:.1}", self.mean, self.p99, self.worst, self.worst_at, self.bytes)
    }
}

/// Where a run dumps each block's least time and bytes, `BLOCKS` naming a
/// directory for a file a measurement.
fn dump_path(label: &str, coder: &str) -> Option<PathBuf> {
    let dir = std::env::var_os("BLOCKS")?;
    let safe: String = label.chars().map(|c| if c.is_ascii_alphanumeric() || c == '-' || c == '.' { c } else { '_' }).collect();
    Some(PathBuf::from(dir).join(format!("{safe}-{coder}.tsv")))
}

fn coders(shape: Shape) -> Vec<(&'static str, Box<dyn Coder>)> {
    let block = shape.block() as u16;
    vec![
        ("libflac-5", Box::new(RefCoder(libflac_ref::Encoder::new(libflac_ref::Stream { rate: shape.rate, channels: shape.channels, bits: shape.bits, block }).unwrap()))),
        ("libflac-0", Box::new(LibFlac { shape, level: 0 })),
        ("libflac-8", Box::new(LibFlac { shape, level: 8 })),
        ("sound-flac", Box::new(Own(sound_flac::Encoder::new(sound_flac::Stream { rate: shape.rate, channels: shape.channels, bits: shape.bits, block }).unwrap()))),
    ]
}

/// A PCM WAV, plain or extensible, its samples of 8 to 24 valid bits in a
/// container of one to four bytes: its shape, at the valid bits, and its
/// samples interleaved, sign-extended to i32.
fn wav(path: &Path) -> (Shape, Vec<i32>) {
    let bytes = std::fs::read(path).unwrap();
    assert!(&bytes[..4] == b"RIFF" && &bytes[8..12] == b"WAVE", "{}: not a WAV", path.display());
    let u16_at = |i: usize| u16::from_le_bytes([bytes[i], bytes[i + 1]]);
    let u32_at = |i: usize| u32::from_le_bytes(bytes[i..i + 4].try_into().unwrap());
    let (mut shape, mut data, mut container) = (None, None, 0u8);
    let mut i = 12;
    while i + 8 <= bytes.len() {
        let (id, len) = (&bytes[i..i + 4], u32_at(i + 4) as usize);
        let body = i + 8;
        match id {
            b"fmt " => {
                let tag = u16_at(body);
                container = u16_at(body + 14) as u8;
                // WAVE_FORMAT_EXTENSIBLE: the sample's valid bits apart from
                // its container's, and the format in the subformat's first
                // two bytes; valid bits of 0 mean the container's.
                let bits = if tag == 0xFFFE {
                    assert!(u16_at(body + 24) == 1, "{}: not PCM", path.display());
                    match u16_at(body + 18) {
                        0 => container,
                        valid => valid as u8,
                    }
                } else {
                    assert!(tag == 1, "{}: not PCM", path.display());
                    container
                };
                shape = Some(Shape { rate: u32_at(body + 4), channels: u16_at(body + 2) as u8, bits });
            }
            b"data" => data = Some(&bytes[body..(body + len).min(bytes.len())]),
            _ => {}
        }
        i = body + len + (len & 1);
    }
    let (shape, data) = (shape.expect("fmt chunk"), data.expect("data chunk"));
    let width = container.div_ceil(8) as usize;
    assert!((1..=4).contains(&width) && shape.bits <= container, "{}: {} bits in {container}", path.display(), shape.bits);
    // The sample left-justified in 32 bits, as its valid bits sit at the top
    // of its container, then down to its valid bits.
    let samples = data
        .chunks_exact(width)
        .map(|s| {
            let left = match width {
                1 => (i32::from(s[0]) - 128) << 24,
                2 => i32::from(i16::from_le_bytes([s[0], s[1]])) << 16,
                3 => i32::from_le_bytes([0, s[0], s[1], s[2]]),
                _ => i32::from_le_bytes([s[0], s[1], s[2], s[3]]),
            };
            left >> (32 - u32::from(shape.bits))
        })
        .collect();
    (shape, samples)
}

fn wavs(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else { return };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            wavs(&path, out);
        } else if path.extension().is_some_and(|e| e.eq_ignore_ascii_case("wav")) {
            out.push(path);
        }
    }
}

fn main() {
    let shapes = [
        Shape { rate: 48_000, channels: 2, bits: 16 },
        Shape { rate: 44_100, channels: 2, bits: 16 },
        Shape { rate: 96_000, channels: 2, bits: 16 },
        Shape { rate: 48_000, channels: 1, bits: 16 },
        Shape { rate: 48_000, channels: 2, bits: 8 },
        Shape { rate: 48_000, channels: 2, bits: 24 },
    ];
    let only: Vec<String> = std::env::args().skip(1).filter(|a| !a.starts_with("coder=")).collect();
    let coder_only: Option<Vec<String>> = std::env::args().find_map(|a| a.strip_prefix("coder=").map(|c| c.split(',').map(str::to_owned).collect()));
    let chosen = |name: &str| coder_only.as_deref().is_none_or(|cs| cs.iter().any(|c| name.contains(c.as_str())));
    let list = |var: &str, all: Vec<String>| -> Vec<String> {
        match std::env::var(var) {
            Ok(v) => v.split(',').filter(|s| !s.is_empty()).map(str::to_owned).collect(),
            Err(_) => all,
        }
    };
    let kinds = list("KINDS", KINDS.iter().map(|s| s.to_string()).collect());
    for shape in shapes {
        let name = shape.name();
        if !only.is_empty() && !only.iter().any(|o| name.contains(o.as_str())) {
            continue;
        }
        for kind in kinds.iter().map(String::as_str) {
            let samples = signal(shape, 50, kind);
            for (coder_name, coder) in coders(shape).iter_mut().filter(|(n, _)| chosen(n)) {
                let measured = measure(shape, &samples, coder.as_mut(), 5, 60, dump_path(&format!("{name}-{kind}"), coder_name));
                println!("shape={name} block={} pcm={} kind={kind} coder={coder_name} {measured}", shape.block(), shape.pcm_bytes());
            }
        }
    }
    let dir = std::env::var("SAMPLES").map(PathBuf::from).unwrap_or_else(|_| PathBuf::from("tmp/samples"));
    let mut files = Vec::new();
    wavs(&dir, &mut files);
    files.sort();
    let wanted = list("FILES", Vec::new());
    for path in files {
        let file = path.strip_prefix(&dir).unwrap_or(&path).to_string_lossy().replace(' ', "_");
        if !wanted.is_empty() && !wanted.iter().any(|w| file.contains(w.as_str())) {
            continue;
        }
        let (shape, samples) = wav(&path);
        if shape.block() == 0 || samples.len() < shape.samples() {
            continue;
        }
        let seconds = samples.len() / shape.samples() / 50;
        for (coder_name, coder) in coders(shape).iter_mut().filter(|(n, _)| chosen(n)) {
            let measured = measure(shape, &samples, coder.as_mut(), 3, 0, dump_path(path.file_stem().unwrap_or_default().to_string_lossy().as_ref(), coder_name));
            println!("shape={} block={} pcm={} file={file} seconds={seconds} coder={coder_name} {measured}", shape.name(), shape.block(), shape.pcm_bytes());
        }
    }
}
