/// Bellman applies a Bellman operation to recommend a new q-value for a state
/// based on the supplied paramters.
/// See [https://en.wikipedia.org/wiki/Bellman_equation](https://en.wikipedia.org/wiki/Bellman_equation)
#[allow(dead_code)]
pub fn bellman(
    old_value: f64,
    learning_rate: f64,
    reward: f64,
    discount_factor: f64,
    optimal_future_value: f64,
) -> f64 {
    learning_rate.mul_add(
        discount_factor.mul_add(optimal_future_value, reward) - old_value,
        old_value,
    )
}

/// Returns a bayesian weighted average where:
///   c = A scalar constant, generally set to a value that represents the
///       minimum number of observations required before an observed parameter
///       begins to be more reliable than the estimated parameter.
///       If c < n, average will favor m.
///       If c > n, average will favor v.
///       If c = n, average will treat v and m with equal weight.
///   n = The number of times parameter of value as been observed.
///   m = An estimated parameter value (typically a mean).
///   v = An observed parameter value.
///   see [https://en.wikipedia.org/wiki/Bayesian_average](https://en.wikipedia.org/wiki/Bayesian_average)
///   see [https://fulmicoton.com/posts/bayesian_rating/](https://fulmicoton.com/posts/bayesian_rating/)
#[allow(dead_code)]
pub fn bayesian_average(c: f64, n: f64, m: f64, v: f64) -> f64 {
    safe_divide(c.mul_add(m, n * v), c + n)
}

/// Returns 0 if the divisor is 0, avoiding div/0 panics.
#[allow(dead_code)]
pub fn safe_divide(dividend: f64, divisor: f64) -> f64 {
    if divisor == 0.0 {
        return 0.0;
    }
    dividend / divisor
}

#[cfg(test)]
mod tests {
    use crate::internal::math;

    #[test]
    fn bellman() {
        let old_value = 0.1;
        let learning_rate = 0.2;
        let reward = 0.3;
        let discount_factor = 0.4;
        let optimal_future_value = 0.5;
        let actual_result = math::bellman(
            old_value,
            learning_rate,
            reward,
            discount_factor,
            optimal_future_value,
        );
        let exp_result = 0.180_000_000_000_000_02;
        assert_eq!(exp_result, actual_result);
    }

    #[test]
    fn safe_divide() {
        let test_cases = vec![(10.0, 2.0, 5.0), (0.0, 2.0, 0.0), (10.0, 0.0, 0.0)];
        for tc in test_cases {
            let result = math::safe_divide(tc.0, tc.1);
            assert_eq!(tc.2, result);
        }
    }
}
