//! Serial Peripheral Interface

use nb;

/// Full duplex (master mode)
///
/// # Notes
///
/// - It's the task of the user of this interface to manage the slave select
///   lines
///
/// - Due to how full duplex SPI works each `send` call must be followed by a
///   `read` call to avoid overruns.
///
/// - Some SPIs can work with 8-bit *and* 16-bit words. You can overload this
///   trait with different `Word` types to allow operation in both modes.
pub trait FullDuplex<Word> {
    /// An enumeration of SPI errors
    ///
    /// Possible errors
    ///
    /// - *overrun*, the shift register was not `read` between two consecutive
    ///   `send` calls.
    type Error;

    /// Reads the word stored in the shift register
    ///
    /// **NOTE** A word must be sent to the slave before attempting to call this
    /// method.
    fn read(&mut self) -> nb::Result<Word, Self::Error>;

    /// Sends a word to the slave
    fn send(&mut self, word: Word) -> nb::Result<(), Self::Error>;
}

/// Clock polarity
#[derive(Clone, Copy, PartialEq)]
pub enum Polarity {
    /// Clock signal low when idle
    IdleLow,
    /// Clock signal high when idle
    IdleHigh,
}

/// Clock phase
#[derive(Clone, Copy, PartialEq)]
pub enum Phase {
    /// Data in "captured" on the first clock transition
    CaptureOnFirstTransition,
    /// Data in "captured" on the second clock transition
    CaptureOnSecondTransition,
}

/// SPI mode
#[derive(Clone, Copy, PartialEq)]
pub struct Mode {
    /// Clock polarity
    pub polarity: Polarity,
    /// Clock phase
    pub phase: Phase,
}
