
<!-- README.md is generated from README.Rmd. Please edit that file -->

# `uwu` (ꈍᴗꈍ)♡

<!-- badges: start -->

<!-- badges: end -->

## Installation

You can install the development version of uwu like so:

This requires Rust to be installed.

``` r
if (requireNamespace("pak")) install.packages("pak")
pak::pak("josiahparry/uwu")
```

## Example

Generate v4 UUIDs ~20x faster than `uuid`

``` r
library(uwu)
new_v4(10)
#>  [1] "72b42022-0727-4388-ab7b-1be064c35b13"
#>  [2] "b90a55d1-b639-4961-be8f-c2efacfaad91"
#>  [3] "e2f04569-8d28-41c4-9082-bb0a657fad48"
#>  [4] "2b178417-ae54-4735-a533-c22b8d21efcd"
#>  [5] "b0a5f44b-6f69-48fc-afa1-119f5fbedfb3"
#>  [6] "a0da601f-869f-452d-898e-da27e55b0255"
#>  [7] "d48549f3-9ea7-4603-8320-844f051c15b8"
#>  [8] "eac9a247-1937-44e8-a38b-b6c3307a3231"
#>  [9] "73b57111-5fcc-463a-bc9b-6ab3bf9616eb"
#> [10] "bde8b03d-6f9b-4b46-a5a6-8d8175c8bd93"
```

Or you can generate v7 UUIDs ~2.5x slower than `uuid`

``` r
new_v7(10)
#>  [1] "01917f06-ca65-7b8b-89cf-0a7866f02bdf"
#>  [2] "01917f06-ca65-71e4-9e42-ca5533d77af1"
#>  [3] "01917f06-ca65-75df-9dad-2bbc831153aa"
#>  [4] "01917f06-ca65-7c8a-bc27-7cb051d3d720"
#>  [5] "01917f06-ca65-7b0b-add0-5151ea3bd095"
#>  [6] "01917f06-ca65-7aa6-9155-766f44342f4d"
#>  [7] "01917f06-ca65-7ad5-8842-575a6dddf65c"
#>  [8] "01917f06-ca65-797a-ba47-4f27567c76df"
#>  [9] "01917f06-ca65-7ceb-b4c7-d53ef56cc5fd"
#> [10] "01917f06-ca65-7ee3-8f82-0f5fd5b7916e"
```

The neat part, though, is that we can impute a UUID into a character
vector by reference. This means that once the operation has occured, you
cannot take it back!!

``` r
x <- c("a", NA, "c")

# x has been imputed 
impute_uuid(x)

x
#> [1] "a"                                   
#> [2] "04f7091d-280d-4007-b46e-23ec313bce7f"
#> [3] "c"
```

It can also have a prefix added to it:

``` r
x <- c("a", NA, "c")

# x has been imputed 
impute_uuid(x, prefix = "NA_")
x
#> [1] "a"                                      
#> [2] "NA_40dd8ec7-eea7-443b-8b10-4fe347525325"
#> [3] "c"
```

Here’s an example to address the problem that prompted this.

``` r
# define a data.frame with missing values 
to_impute <- data.frame(
  col_a = c("one", "two", NA),
  col_b = c("a", NA, "c"),
  col_c = c(NA, "and", "me")
)

df_names <- colnames(to_impute)

for (j in seq_along(to_impute)) {
    impute_uuid(to_impute[[j]], paste0("NA_", df_names[j], "_"))
}

to_impute
#>                                           col_a
#> 1                                           one
#> 2                                           two
#> 3 NA_col_a_a7b19b78-fa3f-4288-9216-368698d5ef2c
#>                                           col_b
#> 1                                             a
#> 2 NA_col_b_cdbe1ad4-9afb-41a4-b6c9-d918de5d2f7d
#> 3                                             c
#>                                           col_c
#> 1 NA_col_c_84fc3180-a9f6-403a-aeb2-1d727dad3126
#> 2                                           and
#> 3                                            me
```

### Short uuids:

Generate short uuid string (default flickrBase58 alphabet)

``` r
n <- 10
short_uuid(n)
#>  [1] "qJdBG92hJ92XmoFTMkgiU6" "5eRk6V38GxLMVA9agqUP8F" "jwb46RHtsopS5a8gAR1voR"
#>  [4] "uH2Q4qpq3sJ7wofq44hgZU" "grZX5JNrumACkniS85uU8q" "oFPAZvVXdSdXaEqauEhvQP"
#>  [7] "qRxN3FaW8vGR29PZccHa1b" "388WbhZFJ46jqZwNqXVhMC" "3ZyBbPkfo4xMimtZr54zq9"
#> [10] "6kaSVP3aLnGSuKPhtEVUAU"
```

Bitcoin58 example:

``` r
short_uuid(n, "bitcoin58")
#>  [1] "2vJfHgjht7p81AWP8yy7aB" "YTZfzbaumai2EYqtuMSYgi" "DMj3oJku2f12CzmiwGgDny"
#>  [4] "NExzMZaujuoqWKsDvGy5Dj" "NY9Dgrh1FmfYUutbHKBKFQ" "KFzk6QP4y35JNbkkJ8Cuhb"
#>  [7] "Cko3iTGECRKJtcr2vomYJu" "6nvBkixtMeqDeV2H5ar42G" "FfRN4zDFVqMnorRrRnRa8G"
#> [10] "Ud7m4PLxdmKQgx4t8iwBbp"
```

Convert uuid V4 to short (default flickrBase58 alphabet)

``` r
uuids <- new_v4(n)
uuid_to_short(uuids)
#>  [1] "nrPm6baA9xY1Ywct3QzwaZ" "mArHz54ZubSEBu9tj7Qk6e" "vUL49aJG2X63sUS5Tn7j5j"
#>  [4] "dLkjGYh3EMQNLYPjTxfkSr" "qmw9shLXXdNr5wn4dmqw6J" "kWgBv8U57u5iGuFZLU68xw"
#>  [7] "ks8msRngAtSGxSdqXiVJQU" "pW5VVTwaXpeANEf5QPbC87" "k77xVKYfyu511nE96ZU4ra"
#> [10] "5bBuDDB2yB8UiuWCqp6XLK"
```

Convert uuid V4 to short (default bitcoin58 alphabet)

``` r
uuid_to_short(uuids, "bitcoin58")
#>  [1] "NSpM6BAb9Yy1yXCU3qaXAz" "MbSia54zVBsfcV9UK7qL6E" "Wum49Ajh2x63Tus5tN7K5K"
#>  [4] "DmLKhyH3fnqomypKtYFLsS" "RMX9THmxxDoS5XN4DMRX6j" "LwGcW8u57V5JhVgzmu68YX"
#>  [7] "LT8MTrNGbUshYsDRxJvjqu" "Qw5vvtXAxQEbofF5qpBd87" "L77YvkyFZV511Nf96zu4SA"
#> [10] "5BcVeec2Zc8uJVwdRQ6xmk"
```
