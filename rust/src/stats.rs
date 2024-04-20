use std::f64::consts::PI;

/// A trait for calculations using a specific statistical distribution.
///
/// Source: https://www.quantstart.com/articles/Statistical-Distributions-in-C/
pub trait StatisticalDistribution {
    /// The probability density function.
    fn pdf(&self, x: f64) -> f64;
    /// The cumulative distrubition function.
    fn cdf(&self, x: f64) -> f64;
    /// The inverse cumulative distribution function.
    fn inv_cdf(&self, quantile: f64) -> f64;
    /// The mean of the distribution.
    fn mean(&self) -> f64;
    /// The variance of the distribution.
    fn var(&self) -> f64;

    /// The standard deviation of the distribution.
    fn stdev(&self) -> f64 {
        self.var().sqrt()
    }
}

/// A standard normal distribution (mean = 0, standard deviation = 1).
/// Implements the StatisticalDistribution trait.
///
/// Source: https://www.quantstart.com/articles/Statistical-Distributions-in-C/
#[derive(Clone, Copy, Default)]
pub struct StandardNormalDistribution;

pub type SND = StandardNormalDistribution;

impl StandardNormalDistribution {
    /// Creates a new StandardNormalDistribution.
    pub const fn new() -> Self {
        Self
    }
}

impl StatisticalDistribution for StandardNormalDistribution {
    fn pdf(&self, x: f64) -> f64 {
        (1.0 / (2.0 * PI).sqrt()) * (-0.5 * x * x).exp()
    }

    fn cdf(&self, x: f64) -> f64 {
        let k = 1.0 / (1.0 + 0.2316419 * x);
        let k_sum = k
            * (0.319381530
                + k * (-0.356563782 + k * (1.781477937 + k * (-1.821255978 + 1.330274429 * k))));

        if x >= 0.0 {
            1.0 - (1.0 / (2.0 * PI).sqrt()) * (-0.5 * x * x).exp() * k_sum
        } else {
            1.0 - self.cdf(-x)
        }
    }

    fn inv_cdf(&self, quantile: f64) -> f64 {
        // This is the Beasley-Springer-Moro algorithm which can
        // be found in Glasserman [2004]. We won't go into the
        // details here, so have a look at the reference for more info
        const A: [f64; 4] = [
            2.50662823884,
            -18.61500062529,
            41.39119773534,
            -25.44106049637,
        ];

        const B: [f64; 4] = [
            -8.47351093090,
            23.08336743743,
            -21.06224101826,
            3.13082909833,
        ];

        const C: [f64; 9] = [
            0.3374754822726147,
            0.9761690190917186,
            0.1607979714918209,
            0.0276438810333863,
            0.0038405729373609,
            0.0003951896511919,
            0.0000321767881768,
            0.0000002888167364,
            0.0000003960315187,
        ];

        if quantile >= 0.5 && quantile <= 0.92 {
            let (mut num, mut denom) = (0.0, 1.0);
            for i in 0..4 {
                num += A[i] * (quantile - 0.5).powi(2 * i as i32 + 1);
                denom += B[i] * (quantile - 0.5).powi(2 * i as i32);
            }
            num / denom
        } else if quantile > 0.92 && quantile < 1.0 {
            let mut num = 0.0;
            for i in 0..9 {
                num += C[i] * (-(1.0 - quantile).ln()).ln().powi(i as _);
            }
            num
        } else {
            -1.0 * self.inv_cdf(1.0 - quantile)
        }
    }

    fn mean(&self) -> f64 {
        0.0
    }

    fn var(&self) -> f64 {
        1.0
    }

    fn stdev(&self) -> f64 {
        1.0
    }
}

/// Calculate the normal CDF using the StandardNormalDistribution.
#[inline(always)]
pub fn norm_cdf(x: f64) -> f64 {
    SND::new().cdf(x)
}
