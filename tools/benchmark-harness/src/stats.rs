//! Deterministic robust statistics used by capture and calibration.

/// Median of a non-empty finite sample.
#[must_use]
pub fn median(values: &[f64]) -> f64 {
    assert!(!values.is_empty(), "median requires at least one value");
    let mut sorted = values.to_vec();
    sorted.sort_by(f64::total_cmp);
    let middle = sorted.len() / 2;
    if sorted.len().is_multiple_of(2) {
        (sorted[middle - 1] + sorted[middle]) / 2.0
    } else {
        sorted[middle]
    }
}

/// Median absolute deviation from the sample median.
#[must_use]
pub fn mad(values: &[f64]) -> f64 {
    let center = median(values);
    let deviations: Vec<f64> = values.iter().map(|value| (value - center).abs()).collect();
    median(&deviations)
}

/// Nearest-rank percentile for a non-empty finite sample.
#[must_use]
pub fn nearest_rank(values: &[f64], percentile: f64) -> f64 {
    assert!(!values.is_empty(), "percentile requires at least one value");
    assert!((0.0..=1.0).contains(&percentile), "percentile must be within [0, 1]");
    let mut sorted = values.to_vec();
    sorted.sort_by(f64::total_cmp);
    let rank = (percentile * sorted.len() as f64).ceil().max(1.0) as usize;
    sorted[rank - 1]
}

/// Whether two recomputed statistics differ by at most a few ULPs at the computation scale.
#[must_use]
pub fn approximately_equal(left: f64, right: f64, computation_scale: f64) -> bool {
    left.is_finite()
        && right.is_finite()
        && computation_scale.is_finite()
        && (left - right).abs()
            <= f64::EPSILON * left.abs().max(right.abs()).max(computation_scale.abs()).max(1.0) * 8.0
}

#[cfg(test)]
mod tests {
    use super::{approximately_equal, mad, median, nearest_rank};

    #[test]
    fn should_calculate_exact_odd_median_and_mad() {
        let values = [1.0, 1.5, 2.0, 8.0, 9.0];
        assert_eq!(median(&values), 2.0);
        assert_eq!(mad(&values), 1.0);
    }

    #[test]
    fn should_use_nearest_rank_for_p95() {
        let values: Vec<f64> = (1..=20).map(f64::from).collect();
        assert_eq!(nearest_rank(&values, 0.95), 19.0);
    }

    #[test]
    fn should_accept_round_trip_statistic_within_a_few_ulps() {
        assert!(approximately_equal(
            2.278_750_000_000_000_5,
            2.278_750_000_000_001,
            2.278_750_000_000_001
        ));
        assert!(!approximately_equal(2.278_750_000_000_000_5, 2.3, 2.3));
    }
}
