module Stats

export StatisticalDistribution,
    StandardNormalDistribution, SND,
    pdf, cdf, inv_cdf, mean, var, stdev

# Source: https://www.quantstart.com/articles/Statistical-Distributions-in-C/
abstract type StatisticalDistribution end

struct StandardNormalDistribution <: StatisticalDistribution end

const SND = StandardNormalDistribution()

function pdf(_::StandardNormalDistribution, x::Float64)::Float64
    (1.0 / sqrt(2.0 * pi)) * exp(-0.5 * x^2)
end

function cdf(dist::StandardNormalDistribution, x::Float64)::Float64
    k = 1.0 / (1.0 + 0.2316419 * x)
    k_sum = k * (0.319381530 +
             k * (-0.356563782 +
                  k * (1.781477937 + k * (-1.821255978 + 1.330274429 * k))))
    if x >= 0.0
        1.0 - (1.0 / sqrt(2.0 * pi)) * exp(-0.5 * x^2) * k_sum
    else
        1.0 - cdf(dist, -x)
    end
end

function inv_cdf(dist::StandardNormalDistribution, quantile::Float64)::Float64
    # This is the Beasley-Springer-Moro algorithm which can
    # be found in Glasserman [2004]. We won't go into the
    # details here, so have a look at the reference for more info
    A = [2.50662823884, -18.61500062529, 41.39119773534,
                          -25.44106049637]

    B = [-8.47351093090, 23.08336743743, -21.06224101826,
    3.13082909833]

    C = [
        0.3374754822726147, 0.9761690190917186, 0.1607979714918209,
        0.0276438810333863, 0.0038405729373609, 0.0003951896511919,
        0.0000321767881768, 0.0000002888167364, 0.0000003960315187]

    if quantile >= 0.5 && quantile <= 0.92
        num, denom = 0.0, 1.0
        for i in 1:4
            num += A[i] * (quantile - 0.5)^(2 * i)
            denom += B[i] * (quantile - 0.5)^(2 * (i - 1))
        end
        num / denom
    elseif quantile > 0.92 && quantile < 1
        num = 0.0
        for i in 1:9
            num += C[i] * log(-log(1 - quantile))^(i - 1)
        end
        num
    else
        -1.0 * inv_cdf(dist, 1 - quantile)
    end
end

function mean(_::StandardNormalDistribution)::Float64
    0.0
end

function var(_::StandardNormalDistribution)::Float64
    1.0
end

function stdev(_::StandardNormalDistribution)::Float64
    1.0
end

norm_pdf(x::Float64) = cdf(SND, x)
norm_cdf(x::Float64) = cdf(SND, x)
norm_inv_cdf(x::Float64) = cdf(SND, x)

end
