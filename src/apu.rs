pub struct Apu {
    channel1: Channel1,
    channel2: Channel2,
}

struct Channel1 {
    // Sweep
    sweep_time: u8,
    sweep_trend: SoundTrend,
    sweep_shift: u8,

    wave_pattern_duty: u8,
    length_counter: u8,

    // Envelope
    initial_volume_envelope: u8,
    envelope_direction: SoundTrend,
    envelope_sweep: u8,

    // Frequency
    frequency: u16,
    counter_enabled: bool,
}

struct Channel2 {
    wave_pattern_duty: u8,
    length_counter: u8,

    // Envelope
    initial_volume_envelope: u8,
    envelope_direction: SoundTrend,
    envelope_sweep: u8,

    // Frequency
    frequency: u16,
    counter_enabled: bool,
}

enum SoundTrend {
    Increasing,
    Decreasing,
}
