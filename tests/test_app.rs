use logvolumegenerator::RuntimeError;
use std::time::Duration;

#[test]
pub fn test_correct_duration_with_valid_params() {
    let result = logvolumegenerator::calculate_iteration_pause(200, 100, 125);

    assert_eq!(
        result,
        Ok(Duration::from_secs_f64(1f64 / ((200f64 / 125f64) / 100f64)))
    )
}

#[test]
pub fn test_correct_duration_with_invalid_params() {
    let cases = vec![(0, 200, 125), (200, 0, 0), (200, 200, 0)];
    for (out_bytes, out_secs, entry_bytes) in cases {
        let result =
            logvolumegenerator::calculate_iteration_pause(out_bytes, out_secs, entry_bytes);
        assert_eq!(result, Err(RuntimeError::InvalidArgument));
    }
}
