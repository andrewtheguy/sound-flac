//! FLAC for a desktop's sound, as wlshare and the zDesk gateway both code it.
//!
//! wlshare sends what its desktop plays as FLAC, and so does the gateway for a
//! desktop it reaches itself, so the sound is lossless on the way to whoever
//! listens. This crate is the one place that sound is made FLAC:
//!
//! - **[`Stream`]** is what the two ends agree on before any sound: the rate,
//!   the channels, the sample width and the block, the frames of samples in
//!   every FLAC frame.
//! - **[`Encoder`]** makes one FLAC frame of every block, the moment the block
//!   is whole.
//!
//! Nothing here decodes: the listeners are a browser's own FLAC decoder and
//! tools of their own, and this crate is only ever the sender's.
//!
//! The encoder is this crate's own, in plain Rust: building compiles no C,
//! needs no libFLAC installed and loads nothing at run time, and a user ships
//! no library beside its binary.
//!
//! ## Every frame on its own
//!
//! A frame is made of its block alone, with nothing of the blocks before it,
//! and is numbered zero: a decoder takes each frame as it comes, so a frame
//! lost on the way costs its own samples and nothing after. The stream header,
//! `STREAMINFO`, is never sent either: everything in it is in the [`Stream`]
//! both ends hold, so a decoder builds it ([`Stream::streaminfo`]) and reads
//! each frame behind it. A frame states its own rate and width, and leaves
//! either to that header only when no frame header has a code for it.
//!
//! ## Tuned for sound heard as it is made
//!
//! A channel is predicted with FLAC's fixed polynomials, of the order up to
//! four that leaves the smallest residual, or with a linear predictor of up
//! to order 8 fitted to it by autocorrelation through a Tukey window and
//! Levinson-Durbin as libFLAC's default level does, its order chosen with the
//! error its coefficients' rounding adds weighed exactly, whichever writes
//! fewer bits. A stereo pair is chosen every fourth block: each of left, right,
//! mid and side is estimated from its autocorrelation, the mid's following
//! from the other three's, and where one pair is estimated under every other
//! by a quarter of a bit a sample it is taken; otherwise all four channels are
//! counted, the fit's residual taken and counted in one Rice partition, a
//! true count, and the fixed predictor's beside it where the fit is not
//! clear, the pair whose two counts are least is taken, and its two channels
//! are finished, the partitions searched. That is libFLAC's choice, made on
//! counts within the partitioning's gain of the real bits rather than on the
//! estimates, which cannot tell left and right from mid and side on a piano.
//! On the three blocks between, the pair the last block was coded with is coded
//! again, each channel the fit first: where the fit is estimated to beat the
//! best fixed predictor by an eighth of a bit a sample, after a correction the
//! channel learned the last time both were coded, the fit alone is coded, and
//! otherwise both are and compared; each channel of the pair has both coded
//! once in two cycles whatever the estimate, on the two blocks after every
//! other choice, so that its correction is of this second; the two other channels are not looked
//! at past a constant check and a bound, and another pair is taken only where
//! it is certainly cheaper. A channel on its own, mono or more than two, is
//! coded the fit first on every block. The fit is skipped where the block is
//! noise by the fixed predictors and the last fit was not estimated to win, as
//! on noise and a quiet desktop, and where the fit, coded beside the fixed
//! coding, has lost to it by an eighth of a bit a sample two blocks running,
//! as on 8-bit music, where the fixed coding wins every block; either skip is
//! for seven blocks and the fit is tried again. The fixed path is left out
//! the same way where the fit, coded beside it, has won by a sixteenth of a
//! bit a sample two blocks running, as on the side channel of a pure tone,
//! where the fit wins every block by just under the margin, but for the two
//! blocks in eight the schedule has both coded on. A residual is Rice-coded in
//! the partitions that cost least. Bits every sample of a channel has zero at the bottom, as
//! 8-bit content in a 16-bit stream does, are shifted out as FLAC's wasted
//! bits. A channel that is one value is a constant, and one nothing shortens
//! goes as it is. Every pass over the samples is a plain loop the compiler
//! vectorises at a target's baseline, the autocorrelation in double precision
//! in one pass of lanes that stay in registers, its multiply and add fused
//! into one instruction where the machine has it, the residual of a sample wider
//! than 16 bits in two 16-bit passes, and the Rice codes go out four, three or
//! two to a word as the parameter lets them.
//!
//! Measured on 20 ms blocks of 16-bit stereo at 48 kHz, one core of an AMD
//! EPYC Rome at the x86-64 baseline: a block of music takes about 10 µs and
//! is 48% of its PCM, speech 6 µs and 30%, a pure tone 10 µs and 26%, noise
//! 7 µs and 76%, silence a microsecond and sixteen bytes. libFLAC at its
//! default level, which this replaced, took 34, 29, 33, 33 and 20 µs for the
//! same bytes within a tenth of a per cent, and 1% more on the tone. Mono at
//! 48 kHz is 5 µs against libFLAC's 20, and 44.1 kHz stereo 9 against 31. On
//! recordings, 0.22 to 0.44 of its time a block for its bytes within 0.2%,
//! the block that chooses the pair afresh, every fourth, a quarter to a
//! third as long again as a carried one. On the i5-8500T of the runs before
//! every block of this encoder's was half as long again, libFLAC's a quarter.
//!
//! Nothing about a wire is here: how a frame is framed in a message, how the
//! stream's shape is agreed and what a sample's bytes are on either side belong
//! to the user.

mod bits;
mod lpc;
mod subframe;

use bits::{BitWriter, crc8, crc16};
use lpc::Window;
use subframe::{Channel, PAIR};
use thiserror::Error;

/// The bytes of a `STREAMINFO` block.
pub const STREAMINFO_LEN: usize = 34;

/// What both ends agree on before any sound.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Stream {
    /// Samples per second per channel, 1 to 655 350, the most a frame header
    /// can state.
    pub rate: u32,
    /// 1 to 8.
    pub channels: u8,
    /// The bits in a sample, 4 to 24.
    pub bits: u8,
    /// The frames of samples in every FLAC frame, 16 or more.
    pub block: u16,
}

impl Stream {
    /// Whether this is a stream FLAC carries and the encoder codes: FLAC's own
    /// limits, but for a width past 24 bits, whose residuals outgrow 32.
    pub fn check(&self) -> Result<(), Error> {
        let coded = (1..=655_350).contains(&self.rate)
            && (1..=8).contains(&self.channels)
            && (4..=24).contains(&self.bits)
            && self.block >= 16;
        if coded { Ok(()) } else { Err(Error::Unsupported(*self)) }
    }

    /// The samples in a block: a frame of them for each channel.
    pub fn samples(&self) -> usize {
        usize::from(self.block) * usize::from(self.channels)
    }

    /// The stream header a decoder reads frames behind, as the FLAC
    /// specification lays out `STREAMINFO`: the block as both the smallest and
    /// the largest, and the frame sizes, the total and the MD5 unknown.
    pub fn streaminfo(&self) -> Result<[u8; STREAMINFO_LEN], Error> {
        self.check()?;
        let block = self.block.to_be_bytes();
        let mut info = [0u8; STREAMINFO_LEN];
        info[0..2].copy_from_slice(&block);
        info[2..4].copy_from_slice(&block);
        // Rate (20 bits), channels - 1 (3), bits per sample - 1 (5), total
        // samples (36, unknown).
        let packed =
            (u64::from(self.rate) << 44) | (u64::from(self.channels - 1) << 41) | (u64::from(self.bits - 1) << 36);
        info[10..18].copy_from_slice(&packed.to_be_bytes());
        Ok(info)
    }

    /// The frame header's block size code, and the bytes that follow the
    /// frame number to state it when the code does not.
    fn block_code(&self) -> (u8, Vec<u8>) {
        let less_one = self.block - 1;
        match self.block {
            192 => (1, vec![]),
            576 => (2, vec![]),
            1152 => (3, vec![]),
            2304 => (4, vec![]),
            4608 => (5, vec![]),
            256 => (8, vec![]),
            512 => (9, vec![]),
            1024 => (10, vec![]),
            2048 => (11, vec![]),
            4096 => (12, vec![]),
            8192 => (13, vec![]),
            16384 => (14, vec![]),
            32768 => (15, vec![]),
            ..=256 => (6, vec![less_one as u8]),
            _ => (7, less_one.to_be_bytes().to_vec()),
        }
    }

    /// The frame header's sample rate code, and the bytes that state the rate
    /// when the code does not; zero leaves it to `STREAMINFO`.
    fn rate_code(&self) -> (u8, Vec<u8>) {
        match self.rate {
            88_200 => (1, vec![]),
            176_400 => (2, vec![]),
            192_000 => (3, vec![]),
            8_000 => (4, vec![]),
            16_000 => (5, vec![]),
            22_050 => (6, vec![]),
            24_000 => (7, vec![]),
            32_000 => (8, vec![]),
            44_100 => (9, vec![]),
            48_000 => (10, vec![]),
            96_000 => (11, vec![]),
            r if r % 1000 == 0 && r / 1000 <= 255 => (12, vec![(r / 1000) as u8]),
            r if r <= 65_535 => (13, (r as u16).to_be_bytes().to_vec()),
            r if r % 10 == 0 && r / 10 <= 65_535 => (14, ((r / 10) as u16).to_be_bytes().to_vec()),
            _ => (0, vec![]),
        }
    }

    /// The frame header's sample size code; zero leaves it to `STREAMINFO`.
    fn size_code(&self) -> u8 {
        match self.bits {
            8 => 1,
            12 => 2,
            16 => 4,
            20 => 5,
            24 => 6,
            _ => 0,
        }
    }
}

/// Why a frame could not be made.
#[derive(Debug, Error)]
pub enum Error {
    #[error("{0:?} is not a stream the encoder codes")]
    Unsupported(Stream),
    #[error("a block of {got} samples, where the stream's is {want}")]
    Block { got: usize, want: usize },
    /// A sample the stream's width does not hold.
    #[error("a sample of {value}, in a stream of {bits} bits")]
    Sample { value: i32, bits: u8 },
}

/// One stream's encoder: a block in, a FLAC frame out.
pub struct Encoder {
    stream: Stream,
    /// The frame header, through the frame number and what follows it, with
    /// the channels coded apart; its CRC is added as it is written.
    head: Vec<u8>,
    /// A channel of the block each, and for a stereo pair the mid and the
    /// side after the two.
    channels: Vec<Channel>,
    /// The window a channel is seen through for its linear predictor's fit.
    window: Window,
    /// The pairing the last block was coded with, the one fitted on a block
    /// that is not every channel's.
    pairing: usize,
    /// The block's place in the schedules, counted from the stream's first
    /// block and wrapping where [`RECHOOSE`] and [`REFRESH`] both do.
    tick: u8,
}

/// Every so many blocks all four channels of a pair are counted and the
/// pair chosen afresh; between, only the pair being carried is coded.
const RECHOOSE: u8 = 4;
/// Every so many blocks each channel of the pair has both codings made and
/// compared whatever the fit is estimated at, the first channel on the
/// block after the boundary and the second on the next, which brings its
/// correction up to date; a multiple or a divisor of [`RECHOOSE`], so that
/// those blocks are carried ones. Measured on the EPYC against once a
/// cycle: once in two, 1.3% to 2.8% fewer instructions on every stereo
/// recording for at most 0.013% more bytes, the orchestra's 0.2 of a byte
/// a block; the pair chosen every eighth block as well took 4.8% to 6.5%
/// off and the late block came half as often, but for 0.07% to 0.11% of
/// the bytes on the pianos, the mixes and the talk, and was not taken.
const REFRESH: u8 = 8;

/// The frame header's channel assignments for a stereo pair, in the order
/// they are weighed.
const PAIRINGS: [u8; 4] = [1, 8, 9, 10];

impl Encoder {
    /// An encoder for `stream`, if it is one the encoder codes.
    pub fn new(stream: Stream) -> Result<Self, Error> {
        stream.check()?;
        let (block_code, block_bytes) = stream.block_code();
        let (rate_code, rate_bytes) = stream.rate_code();
        // The sync code with fixed blocking, the codes, the channels apart
        // and the width, the frame number zero, then what the codes left.
        let mut head = vec![0xFF, 0xF8, (block_code << 4) | rate_code, ((stream.channels - 1) << 4) | (stream.size_code() << 1), 0];
        head.extend(block_bytes);
        head.extend(rate_bytes);
        let candidates = if stream.channels == 2 { 4 } else { usize::from(stream.channels) };
        let block = usize::from(stream.block);
        let precision = lpc::precision(u32::from(stream.bits), block);
        let channels = (0..candidates).map(|_| Channel::new(block, precision)).collect();
        Ok(Self { stream, head, channels, window: Window::new(block), pairing: 0, tick: 0 })
    }

    /// The stream this encoder was made for.
    pub fn stream(&self) -> Stream {
        self.stream
    }

    /// Append to `out` the FLAC frame of one block: [`Stream::samples`]
    /// interleaved samples, each a signed value of the stream's width. Nothing
    /// is appended on an error.
    pub fn encode(&mut self, block: &[i32], out: &mut Vec<u8>) -> Result<(), Error> {
        let stream = self.stream;
        if block.len() != stream.samples() {
            return Err(Error::Block { got: block.len(), want: stream.samples() });
        }
        // A sample outside the width is found by the channels' preparing,
        // which folds the width the samples need in any case: a pass of
        // its own over the block cost 0.8 µs at 960 frames of a pair.
        let bps = u32::from(stream.bits);
        const PAIRS: [(usize, usize); 4] = [(0, 1), (0, 3), (1, 3), (2, 3)];
        let pairing = if stream.channels == 2 {
            let [left, right, mid, side] = &mut self.channels[..] else { unreachable!("a stereo encoder has four candidates") };
            let pairs = block.as_chunks::<2>().0;
            // Wrapping, since a sample outside the width has not been
            // refused yet; the mid and side of a block with one are not used.
            for ((((&[l, r], le), ri), mi), si) in pairs.iter().zip(&mut left.signal).zip(&mut right.signal).zip(&mut mid.signal).zip(&mut side.signal) {
                *le = l;
                *ri = r;
                *mi = l.wrapping_add(r) >> 1;
                *si = l.wrapping_sub(r);
            }
            let window = &self.window;
            if !left.prepare(bps) || !right.prepare(bps) {
                return Err(outside(block, stream.bits));
            }
            mid.prepare(bps);
            side.prepare(bps + 1);
            let tick = self.tick;
            let fresh = tick.is_multiple_of(RECHOOSE);
            let mut channels = [left, right, mid, side];
            // Every fourth block all four channels are weighed from their
            // autocorrelations and the pair chosen on that; on the blocks
            // between, the pair the last block was coded with is coded
            // again, each channel the fit first, and the other two
            // channels are not looked at past the constant check and a
            // bound. Sound changes pair seldom in 20 ms: measured on the
            // recordings, choosing afresh every eighth or sixteenth block
            // instead left the bytes the same within 0.05%. A channel that
            // is one value, the desktop playing the same sound on both
            // channels or on one, is coded as a constant by the check,
            // and the pair moves to it where the bound says it is cheaper.
            let mut best = if !fresh {
                let mut best = self.pairing;
                // Each channel's correction is brought up to date every
                // `REFRESH` blocks, both codings made and compared whatever
                // the fit is estimated at, the first channel on the first
                // carried block after the boundary and the second on the
                // next, so that the cost falls on blocks that are not the
                // fresh one.
                for (k, i) in [PAIRS[best].0, PAIRS[best].1].into_iter().enumerate() {
                    channels[i].code_alone(window, tick % REFRESH == k as u8 + 1);
                }
                // Another pair is taken where it is certainly cheaper: where
                // the most its channels can cost, from one pass over each
                // not coded, is under the pair coded. Measured on the wide
                // stereo mix, the pair moves between the full choices
                // often enough to cost 0.06% of the bytes left alone.
                let mut bits = channels[PAIRS[best].0].bits() + channels[PAIRS[best].1].bits();
                for (p, &(a, b)) in PAIRS.iter().enumerate() {
                    if p != best && channels[a].at_most() + channels[b].at_most() < bits {
                        for i in [a, b] {
                            channels[i].code_alone(window, false);
                        }
                        best = p;
                        bits = channels[a].bits() + channels[b].bits();
                    }
                }
                best
            } else {
                // Every channel that wants a fit has its autocorrelation
                // taken, and is counted from it in one Rice partition, the
                // fit's residual taken and the fixed predictor's where the
                // fit is not clear: a true count, within what the
                // partitioning gains. The mid's autocorrelation
                // follows from the other three's, the mid being half of
                // left plus right and the side their difference: autoc(m) =
                // (2 (autoc(l) + autoc(r)) - autoc(s)) / 4, but for the
                // half sample the mid's rounding loses, which is well under
                // what any block has in it; each is of its channel with the
                // wasted bits shifted out, so each is scaled back first.
                let [left, right, mid, side] = &mut channels;
                let wanted = [left.fit_wanted(), right.fit_wanted(), mid.fit_wanted(), side.fit_wanted()];
                let autoc = |wanted: bool, channel: &mut Channel| if wanted { channel.autocorrelation(window) } else { None };
                let (al, ar, as_) = (autoc(wanted[0], left), autoc(wanted[1], right), autoc(wanted[3], side));
                let scale = |channel: &Channel| 4f64.powi(channel.wasted() as i32);
                let am = match (&al, &ar, &as_) {
                    _ if !wanted[2] => None,
                    (Some(l), Some(r), Some(s)) => {
                        let (sl, sr, ss, sm) = (scale(left), scale(right), scale(side), scale(mid));
                        Some(std::array::from_fn(|i| (2.0 * (l[i] * sl + r[i] * sr) - s[i] * ss) * 0.25 / sm))
                    }
                    _ => mid.autocorrelation(window),
                };
                let mut estimates = [0.0; 4];
                for ((channel, autoc), estimate) in channels.iter_mut().zip([al, ar, am, as_]).zip(&mut estimates) {
                    *estimate = channel.weigh(autoc.as_ref());
                }
                // Where the estimates set one pair under every other by
                // `PAIR` a sample, it is taken and its channels coded;
                // otherwise all four channels are counted in one Rice
                // partition, the fit's residual taken, and the pair is the
                // one whose two counts are least, its two channels
                // finished. Counting all four is what libFLAC's default
                // level does with four codings; chosen on the estimates
                // and coded again where the coded pair's real bits said
                // another was cheaper, the choice coded 1.9 pairs a block
                // on the orchestra and ended on another pair than the
                // first on 15% of the blocks, and on the estimates alone
                // 3.4 pairs for 0.17% fewer bytes on the Fantasy, so the
                // four counts are the whole of that search for a residual
                // pass and a fold a channel. On the piano recordings the
                // estimates never settle it, left and right against mid
                // and side within a twentieth of a bit a sample on 99% of
                // the blocks and wrong on half; on made music they settle
                // every block, on the mixes a quarter to a third.
                let guessed = PAIRS.map(|(a, b)| estimates[a] + estimates[b]);
                let best = least(&guessed);
                let second = (0..PAIRS.len()).filter(|&i| i != best).map(|i| guessed[i]).fold(f64::INFINITY, f64::min);
                let best = if second - guessed[best] >= PAIR * f64::from(stream.block) {
                    best
                } else {
                    for channel in channels.iter_mut() {
                        channel.count();
                    }
                    least(&PAIRS.map(|(a, b)| channels[a].coarse() + channels[b].coarse()))
                };
                let (a, b) = PAIRS[best];
                channels[a].finish();
                channels[b].finish();
                best
            };
            // A frame is never more than its PCM and a header: where the
            // pair chosen has a side that noise fills past the width, left
            // and right go instead, weighed by the most the pair can take,
            // its bits as estimated and half a bit a sample over them.
            let (a, b) = PAIRS[best];
            if best != 0 && channels[a].at_most() + channels[b].at_most() > channels[0].verbatim() + channels[1].verbatim() {
                for i in [0, 1] {
                    channels[i].finish();
                }
                best = 0;
            }
            self.tick = (tick + 1) % RECHOOSE.max(REFRESH);
            self.pairing = best;
            best
        } else {
            // A channel on its own is coded the fit first: there is no pair
            // to choose, so its bits need only be its own.
            let channels = usize::from(stream.channels);
            for (c, channel) in self.channels.iter_mut().enumerate() {
                for (s, &sample) in channel.signal.iter_mut().zip(block[c..].iter().step_by(channels)) {
                    *s = sample;
                }
                if !channel.prepare(bps) {
                    return Err(outside(block, stream.bits));
                }
                channel.code_alone(&self.window, false);
            }
            0
        };

        let start = out.len();
        out.extend_from_slice(&self.head);
        if stream.channels == 2 {
            out[start + 3] = (PAIRINGS[pairing] << 4) | (out[start + 3] & 0x0F);
        }
        let crc = crc8(&out[start..]);
        out.push(crc);
        let mut w = BitWriter::new(out);
        match pairing {
            0 => {
                for channel in &self.channels[..usize::from(stream.channels)] {
                    channel.write(&mut w);
                }
            }
            1 => {
                self.channels[0].write(&mut w);
                self.channels[3].write(&mut w);
            }
            // Right and side go side first.
            2 => {
                self.channels[3].write(&mut w);
                self.channels[1].write(&mut w);
            }
            _ => {
                self.channels[2].write(&mut w);
                self.channels[3].write(&mut w);
            }
        }
        w.align();
        let crc = crc16(&out[start..]);
        out.extend_from_slice(&crc.to_be_bytes());
        Ok(())
    }
}

/// The pair whose cost is least, the first on a tie.
fn least<T: PartialOrd>(costs: &[T; PAIRINGS.len()]) -> usize {
    let mut best = 0;
    for (i, cost) in costs.iter().enumerate() {
        if *cost < costs[best] {
            best = i;
        }
    }
    best
}

/// The error for a block with a sample outside `bits`, naming the first.
fn outside(block: &[i32], bits: u8) -> Error {
    let held = -(1i32 << (bits - 1))..(1i32 << (bits - 1));
    let value = *block.iter().find(|s| !held.contains(s)).expect("a channel found one");
    Error::Sample { value, bits }
}

#[cfg(test)]
mod tests {
    use super::*;

    const STEREO: Stream = Stream { rate: 48_000, channels: 2, bits: 16, block: 960 };

    /// A tone with noise on it, interleaved: every sample of it uses the full
    /// range of the stream's width.
    fn signal(stream: Stream, blocks: usize) -> Vec<i32> {
        let mut seed = 0x2545_f491_u32;
        let mut out = Vec::with_capacity(blocks * stream.samples());
        for n in 0..blocks * usize::from(stream.block) {
            for c in 0..stream.channels {
                seed ^= seed << 13;
                seed ^= seed >> 17;
                seed ^= seed << 5;
                let t = n as f64 / f64::from(stream.rate);
                let tone = (t * 440.0 * (1.0 + f64::from(c)) * std::f64::consts::TAU).sin() * 0.7;
                let noise = (f64::from(seed) / f64::from(u32::MAX) - 0.5) * 0.2;
                let full = ((tone + noise).clamp(-1.0, 1.0) * f64::from(i32::MAX)) as i32;
                out.push(full >> (32 - u32::from(stream.bits)));
            }
        }
        out
    }

    /// A clean tone, the right channel the left turned down, which prediction
    /// shortens a great deal.
    fn tone(stream: Stream, blocks: usize) -> Vec<i32> {
        let full = f64::from((1i32 << (stream.bits - 1)) - 1) * 0.6;
        let mut out = Vec::with_capacity(blocks * stream.samples());
        for n in 0..blocks * usize::from(stream.block) {
            let t = n as f64 / f64::from(stream.rate);
            let left = ((t * 440.0 * std::f64::consts::TAU).sin() * full) as i32;
            for c in 0..stream.channels {
                out.push(if c % 2 == 0 { left } else { left * 3 / 4 });
            }
        }
        out
    }

    /// A frame of every block, as [`Encoder`] makes them.
    fn encode(stream: Stream, samples: &[i32]) -> Vec<Vec<u8>> {
        let mut encoder = Encoder::new(stream).unwrap();
        samples
            .chunks(stream.samples())
            .map(|block| {
                let mut frame = Vec::new();
                encoder.encode(block, &mut frame).unwrap();
                frame
            })
            .collect()
    }

    /// The frames' samples by symphonia's decoder, which shares nothing with
    /// this crate, behind the header [`Stream::streaminfo`] builds.
    fn symphonia(stream: Stream, frames: &[Vec<u8>]) -> Vec<i32> {
        use symphonia_core::codecs::audio::well_known::CODEC_ID_FLAC;
        use symphonia_core::codecs::audio::{AudioCodecParameters, AudioDecoder as _, AudioDecoderOptions};
        use symphonia_core::packet::Packet;
        use symphonia_core::units::{Duration, Timestamp};

        let mut params = AudioCodecParameters::new();
        params.for_codec(CODEC_ID_FLAC).with_extra_data(Box::new(stream.streaminfo().unwrap()));
        let mut decoder =
            symphonia_bundle_flac::FlacDecoder::try_new(&params, &AudioDecoderOptions::default()).unwrap();
        let mut out = Vec::new();
        let mut samples = Vec::<i32>::new();
        for frame in frames {
            let packet =
                Packet::new(0, Timestamp::new(0), Duration::new(u64::from(stream.block)), frame.clone());
            let decoded = decoder.decode(&packet).unwrap();
            assert_eq!(decoded.frames(), usize::from(stream.block));
            assert_eq!(decoded.spec().channels().count(), usize::from(stream.channels));
            decoded.copy_to_vec_interleaved(&mut samples);
            // symphonia scales to 32 bits; back down to the stream's width.
            out.extend(samples.iter().map(|sample| sample >> (32 - u32::from(stream.bits))));
        }
        out
    }

    /// Encoded, then read back by symphonia, bit for bit.
    fn round_trips(stream: Stream, samples: &[i32]) -> Vec<Vec<u8>> {
        let frames = encode(stream, samples);
        assert_eq!(frames.len(), samples.len() / stream.samples(), "{stream:?}");
        assert!(symphonia(stream, &frames) == samples, "{stream:?} did not survive symphonia");
        frames
    }

    /// The header's fields where the FLAC specification puts them, read back
    /// bit by bit rather than with the packing that wrote them.
    #[test]
    fn the_streaminfo_has_the_specified_layout() {
        let info = STEREO.streaminfo().unwrap();
        assert_eq!(&info[0..4], &[0x03, 0xC0, 0x03, 0xC0], "960 at both ends");
        assert_eq!(&info[4..10], &[0; 6], "frame sizes unknown");
        // 48000 = 0x0BB80 in 20 bits, then 001 for two channels, then 01111 for
        // 16 bits, then a total of zero.
        assert_eq!(&info[10..14], &[0x0B, 0xB8, 0x02, 0xF0]);
        assert_eq!(&info[14..34], &[0; 20]);
    }

    /// A stream outside the encoder's limits has no header and no encoder.
    #[test]
    fn a_stream_the_encoder_does_not_code_is_refused() {
        for stream in [
            Stream { channels: 0, ..STEREO },
            Stream { channels: 9, ..STEREO },
            Stream { rate: 0, ..STEREO },
            Stream { rate: 655_351, ..STEREO },
            Stream { bits: 3, ..STEREO },
            Stream { bits: 25, ..STEREO },
            Stream { block: 15, ..STEREO },
        ] {
            assert!(matches!(stream.streaminfo(), Err(Error::Unsupported(s)) if s == stream));
            assert!(matches!(Encoder::new(stream), Err(Error::Unsupported(s)) if s == stream));
        }
    }

    /// Every width with a code and some without, one and two channels and a
    /// spread of rates, coded and uncoded: what the encoder makes is read back
    /// bit for bit by symphonia. 70001 Hz is a rate no frame header can state,
    /// so its frames leave it to the stream header; 70000 is stated in daHz
    /// and 44000 in kHz.
    #[test]
    fn the_encoder_round_trips_bit_for_bit() {
        for bits in [4, 8, 9, 12, 16, 20, 24] {
            for channels in [1, 2] {
                for rate in [8_000, 11_025, 44_000, 44_100, 48_000, 70_000, 70_001, 96_000] {
                    let stream = Stream { rate, channels, bits, block: (rate / 50) as u16 };
                    round_trips(stream, &signal(stream, 4));
                    round_trips(stream, &tone(stream, 2));
                }
            }
        }
    }

    /// The edges of what [`Stream::check`] lets through and the odd shapes in
    /// between: the shortest block, the longest, blocks no power of two
    /// divides far, every channel count, the slowest and the fastest rate, and
    /// every coded block size.
    #[test]
    fn the_limits_and_the_odd_shapes_are_coded() {
        let mut streams = vec![
            Stream { block: 16, ..STEREO },
            Stream { block: 17, ..STEREO },
            Stream { block: 65_535, ..STEREO },
            Stream { block: 997, ..STEREO },
            Stream { block: 1000, ..STEREO },
            Stream { rate: 1, block: 16, ..STEREO },
            Stream { rate: 655_350, ..STEREO },
            Stream { rate: 255_000, ..STEREO },
            Stream { rate: 256_000, ..STEREO },
        ];
        streams.extend((1..=8).map(|channels| Stream { channels, ..STEREO }));
        streams.extend([192, 256, 576, 1152, 2304, 4608, 512, 1024, 2048, 4096, 8192, 16384, 32768].map(|block| Stream { block, ..STEREO }));
        for stream in streams {
            round_trips(stream, &signal(stream, 2));
            round_trips(stream, &tone(stream, 1));
        }
    }

    /// A frame's header says what was agreed, each field read where the FLAC
    /// specification puts it: fixed blocking, the stream's block, rate,
    /// channels and sample width, and the number zero, every frame being on its
    /// own.
    #[test]
    fn a_frame_header_states_the_stream() {
        let frames = encode(STEREO, &signal(STEREO, 2));
        assert_eq!(frames.len(), 2);
        for frame in &frames {
            assert_eq!(&frame[..2], &[0xFF, 0xF8], "the sync code, and fixed blocking");
            assert_eq!(frame[2], 0x7A, "a block size of 16 bits follows the number, at 48 kHz");
            assert!(matches!(frame[3] >> 4, 0x1 | 0x8..=0xA), "two channels, however they are paired");
            assert_eq!(frame[3] & 0x0F, 0x08, "16 bits a sample");
            assert_eq!(frame[4], 0, "numbered zero");
            assert_eq!(&frame[5..7], &959u16.to_be_bytes(), "960 frames, less one");
            assert_eq!(frame[7], crc8(&frame[..7]), "the header's CRC");
            let body = frame.len() - 2;
            assert_eq!(&frame[body..], &crc16(&frame[..body]).to_be_bytes(), "the frame's CRC");
        }
        let small = Stream { block: 200, rate: 44_000, ..STEREO };
        let frame = encode(small, &signal(small, 1)).remove(0);
        assert_eq!(frame[2], 0x6C, "a block size of 8 bits, and a rate in kHz");
        assert_eq!(frame[5], 199);
        assert_eq!(frame[6], 44);
    }

    /// A frame is read on its own: a decoder takes it with the one before it
    /// missing, and takes it first. What the encoder carries from the block
    /// before, the pair to fit first, changes how a block is coded and
    /// nothing a decoder needs.
    #[test]
    fn a_frame_owes_nothing_to_the_blocks_before_it() {
        let samples = signal(STEREO, 3);
        let frames = encode(STEREO, &samples);
        let third = &samples[2 * STEREO.samples()..];
        assert!(symphonia(STEREO, &[frames[2].clone()]) == third);
        let kept = [&samples[..STEREO.samples()], third].concat();
        assert!(symphonia(STEREO, &[frames[0].clone(), frames[2].clone()]) == kept);
    }

    /// A channel's samples are predicted, and a stereo pair is coded as the
    /// pair that is smallest: a block of a tone, the right channel the left
    /// turned down, is well under its PCM.
    #[test]
    fn a_tone_is_compressed() {
        let frames = round_trips(STEREO, &tone(STEREO, 1));
        let pcm = STEREO.samples() * 2;
        assert!(frames[0].len() < pcm / 3, "{} of {pcm} bytes", frames[0].len());
        assert!(matches!(frames[0][3] >> 4, 0x8 | 0x9), "a channel and the side, which is the left's quarter");
    }

    /// Bits every sample has zero at the bottom are shifted out: 8-bit content
    /// in a 16-bit stream costs what it would in an 8-bit one, near enough.
    #[test]
    fn wasted_bits_are_not_sent() {
        let narrow = Stream { bits: 8, ..STEREO };
        let eight: Vec<i32> = signal(narrow, 1);
        let widened: Vec<i32> = eight.iter().map(|&s| s << 8).collect();
        let wide_frame = round_trips(STEREO, &widened).remove(0);
        let narrow_frame = round_trips(narrow, &eight).remove(0);
        // The wide stream fits coefficients of more bits, a few bytes a frame.
        assert!(wide_frame.len() <= narrow_frame.len() + 32, "{} against {}", wide_frame.len(), narrow_frame.len());
    }

    /// Silence, the state a desktop is in most of the time, is a few bytes a
    /// frame rather than the 3840 of its samples; so is any one value.
    #[test]
    fn silence_is_a_few_bytes() {
        let silence = vec![0; STEREO.samples()];
        let frames = round_trips(STEREO, &silence);
        assert!(frames[0].len() < 24, "{} bytes", frames[0].len());
        let dc = vec![-1234; STEREO.samples()];
        let frames = round_trips(STEREO, &dc);
        assert!(frames[0].len() < 24, "{} bytes", frames[0].len());
    }

    /// Noise the width fills is left as it is, no longer than its PCM and a
    /// header.
    #[test]
    fn noise_goes_verbatim() {
        let mut seed = 0x1234_5678u32;
        let noise: Vec<i32> = (0..STEREO.samples())
            .map(|_| {
                seed ^= seed << 13;
                seed ^= seed >> 17;
                seed ^= seed << 5;
                (seed >> 16) as i32 - 32_768
            })
            .collect();
        let frames = round_trips(STEREO, &noise);
        assert!(frames[0].len() <= STEREO.samples() * 2 + 16, "{} bytes", frames[0].len());
    }

    #[test]
    fn a_block_of_another_length_is_refused() {
        let mut encoder = Encoder::new(STEREO).unwrap();
        let mut out = Vec::new();
        let short = vec![0; STEREO.samples() - 2];
        assert!(matches!(encoder.encode(&short, &mut out), Err(Error::Block { got: 1918, want: 1920 })));
        assert!(matches!(encoder.encode(&[], &mut out), Err(Error::Block { got: 0, want: 1920 })));
        assert!(out.is_empty());
    }

    /// A sample the width does not hold is refused, named, and nothing is
    /// appended; the encoder is still good for the block after it.
    #[test]
    fn a_sample_outside_the_width_is_refused() {
        let mut encoder = Encoder::new(STEREO).unwrap();
        let mut out = vec![7];
        let mut block = signal(STEREO, 1);
        block[100] = 32_768;
        assert!(matches!(encoder.encode(&block, &mut out), Err(Error::Sample { value: 32_768, bits: 16 })));
        block[100] = -32_769;
        assert!(matches!(encoder.encode(&block, &mut out), Err(Error::Sample { value: -32_769, bits: 16 })));
        assert_eq!(out, [7]);
        block[100] = -32_768;
        block[101] = 32_767;
        encoder.encode(&block, &mut out).unwrap();
        assert!(symphonia(STEREO, &[out[1..].to_vec()]) == block);
    }
}


