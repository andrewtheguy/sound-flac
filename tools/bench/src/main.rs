//! The crate against libFLAC on 20 ms blocks of a desktop's sound, a line a
//! cell for `tools/tables.py` to make the tables of `docs/benchmarks.md` from:
//!
//!     cargo run --release --manifest-path tools/bench/Cargo.toml > bench.log
//!     python3 -I tools/tables.py bench.log
//!
//! Every shape is coded by libFLAC at its default level 5 as v0.0.4 of this
//! crate ran it, a stream of its own for every frame; by libFLAC at 0, its
//! fastest, and 8, its smallest, the same way; and by this crate. A cell's
//! time is the least of five passes over fifty blocks, each pass timed as a
//! whole, after one pass to warm up; its bytes are the frames' over the fifty.
//! Arguments narrow the shapes by substring (`48k`, `mono`); `KINDS` the
//! sounds (`KINDS=music,tone`); `coder=` the coders (`coder=sound-flac`).
use std::ffi::c_void;
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
        let rate = if self.rate % 1000 == 0 { format!("{}k", self.rate / 1000) } else { format!("{:.1}k", self.rate as f64 / 1000.0) };
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
            // Noise at -20 dBFS: the worst a desktop sends, nothing to predict.
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
}

fn run(name: &str, shape: Shape, kind: &str, samples: &[i32], coder: &mut dyn Coder) {
    let blocks = samples.len() / shape.samples();
    let mut out = Vec::new();
    let mut pass = |out: &mut Vec<u8>| {
        out.clear();
        for block in samples.chunks_exact(shape.samples()) {
            coder.encode(block, out);
        }
    };
    pass(&mut out);
    let bytes = out.len();
    let mut least = f64::INFINITY;
    for _ in 0..5 {
        let mut passes = 0u32;
        let start = Instant::now();
        while start.elapsed().as_millis() < 60 {
            pass(&mut out);
            passes += 1;
        }
        least = least.min(start.elapsed().as_secs_f64() * 1e6 / (passes as f64 * blocks as f64));
    }
    println!(
        "shape={} block={} pcm={} kind={kind} coder={name} us={least:.2} bytes={:.1}",
        shape.name(),
        shape.block(),
        shape.pcm_bytes(),
        bytes as f64 / blocks as f64
    );
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
    let coder_only: Option<String> = std::env::args().find_map(|a| a.strip_prefix("coder=").map(str::to_owned));
    let kinds: Vec<String> = std::env::var("KINDS").map(|k| k.split(',').map(str::to_owned).collect()).unwrap_or_else(|_| KINDS.iter().map(|s| s.to_string()).collect());
    for shape in shapes {
        let name = shape.name();
        if !only.is_empty() && !only.iter().any(|o| name.contains(o.as_str())) {
            continue;
        }
        for kind in kinds.iter().map(String::as_str) {
            let samples = signal(shape, 50, kind);
            let block = shape.block() as u16;
            let mut coders: Vec<(&str, Box<dyn Coder>)> = vec![
                ("libflac-5", Box::new(RefCoder(libflac_ref::Encoder::new(libflac_ref::Stream { rate: shape.rate, channels: shape.channels, bits: shape.bits, block }).unwrap()))),
                ("libflac-0", Box::new(LibFlac { shape, level: 0 })),
                ("libflac-8", Box::new(LibFlac { shape, level: 8 })),
                ("sound-flac", Box::new(Own(sound_flac::Encoder::new(sound_flac::Stream { rate: shape.rate, channels: shape.channels, bits: shape.bits, block }).unwrap()))),
            ];
            for (name, coder) in coders.iter_mut() {
                if coder_only.as_deref().is_some_and(|c| !name.contains(c)) {
                    continue;
                }
                run(name, shape, kind, &samples, coder.as_mut());
            }
        }
    }
}
