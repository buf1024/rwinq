#[inline]
pub(crate) fn shadow(
    last_close: f32,
    open: f32,
    close: f32,
    low: f32,
    high: f32,
) -> (f32, f32, f32, f32) {
    if high == low {
        return (0.0, 0.0, 0.0, 0.0);
    }
    let amp = (high - low) * 100.0 / last_close;
    let base = high - low;
    let is_up = close > open;
    if is_up {
        return (
            amp,
            (high - close) * 100.0 / base,
            (close - open) * 100.0 / base,
            (open - low) * 100.0 / base,
        );
    }
    return (
        amp,
        (high - open) * 100.0 / base,
        (open - close) * 100.0 / base,
        (close - low) * 100.0 / base,
    );
}
