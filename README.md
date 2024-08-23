
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
#>  [1] "56d22ce5-943e-43e7-aca5-82d0a12c0777"
#>  [2] "e81b6e1c-bf9c-43ad-990a-750c3b898bc6"
#>  [3] "46ded4ea-6b0b-490a-96c0-dec646ae8c3e"
#>  [4] "0a03c217-7f26-449a-a823-1b65856340ef"
#>  [5] "404b6536-2813-49f9-9144-302ff545c25f"
#>  [6] "7c078703-ed26-448c-a491-064b83c0a0e1"
#>  [7] "d4e7f91b-4989-42c0-83f2-13a38fe71837"
#>  [8] "f8acb97c-3202-495a-97be-f10f0d31b60a"
#>  [9] "75f57e4f-012c-4f2f-aa7f-31294ccf534d"
#> [10] "cf7805db-2ca2-4164-9256-f68c29e39763"
```

Or you can generate v7 UUIDs ~2.5x slower than `uuid`

``` r
new_v7(10)
#>  [1] "01917f26-374a-7006-8ce9-4c0d8a575942"
#>  [2] "01917f26-374a-7700-a4b2-eefed847f094"
#>  [3] "01917f26-374a-75a4-b93d-07dce09bbe2d"
#>  [4] "01917f26-374a-76cf-a9e9-e1b693ed36b4"
#>  [5] "01917f26-374a-71f8-9cda-7bc2a4bf9fa9"
#>  [6] "01917f26-374a-7c4d-a05e-b7e7dbde2e1f"
#>  [7] "01917f26-374a-795a-8a63-94d921e2bbe5"
#>  [8] "01917f26-374a-7c6d-a980-096b16d102a3"
#>  [9] "01917f26-374a-7597-89a1-e8ba4a652a9f"
#> [10] "01917f26-374a-7f9f-8873-f22b6588e3c1"
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
#> [2] "151e4170-cc2e-4174-8444-5137ca33b071"
#> [3] "c"
```

It can also have a prefix added to it:

``` r
x <- c("a", NA, "c")

# x has been imputed 
impute_uuid(x, prefix = "NA_")
x
#> [1] "a"                                      
#> [2] "NA_c71ddf0b-f1c1-4360-ab35-8248738d9615"
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
#> 3 NA_col_a_06f3f6de-72f9-4a26-ba4d-d5cefcfde4ea
#>                                           col_b
#> 1                                             a
#> 2 NA_col_b_d7ec0ff7-e2a6-4e83-9363-d8dadc432c4e
#> 3                                             c
#>                                           col_c
#> 1 NA_col_c_e33563ed-49bc-45c4-afe2-9cbe4ecd1ea3
#> 2                                           and
#> 3                                            me
```

### Short uuids:

Generate short uuid string (default flickrBase58 alphabet)

``` r
n <- 10
new_short(n)
#>  [1] "jM7VgSV2VYt8wc9ZT7F1zy" "qJv5T1c8K3hQZbqToWZpjU" "iMax644mioxtHzsArWvwZd"
#>  [4] "bbSMcMexnxqpHMTz4ocjXs" "q8zvhR4kutDHusxxGHBqPP" "e5BqvbqDjaDFRPepLq8so6"
#>  [7] "iLs21K7a8w9ByVSRsHm7eL" "8M5dqxDwNSGAqjWMyPHJ8x" "9bM64yZFTXK8Y7ikNzG2oC"
#> [10] "wX6hf5PwjJPn2779jnghVE"
```

Bitcoin58 example:

``` r
new_short(n, "bitcoin58")
#>  [1] "QyS5pt4wtYHMKHAMEKSBog" "5yAa9QhgGAWXr41Xpcm8SK" "1EETorZosyQVqDprH2H1Uk"
#>  [4] "MdbtuDcnfir8QX3kV7Lfhq" "DJyE9sre8uZbF9ehFaw9UE" "TQgAWEgMutBWowNqmTp52X"
#>  [7] "Cs8HdRn4VGroqmt4R3XwHc" "W6Kq4QSamjH9jWPahjw6ME" "MaDd65PVLREKZ1Rvqxtedz"
#> [10] "RQpPdk33XbhrN9vuf8Xe39"
```

Convert uuid V4 to short (default flickrBase58 alphabet)

``` r
uuids <- new_v4(n)
uuids
#>  [1] "7da0fa53-e865-4df7-ab8c-593102952d8d"
#>  [2] "ac4d26ae-a444-4b18-bb6b-4af5e7e6c689"
#>  [3] "34d8d78e-d839-4a5d-95a6-701f7364c987"
#>  [4] "f79a3df1-13e7-4f50-b510-65b1ccda6728"
#>  [5] "f624974e-3ec7-4eab-82a7-ec7b259b5a78"
#>  [6] "f22b10b5-4738-401c-a22f-e13191407823"
#>  [7] "e53c96cc-ff7b-4786-8759-df8f66af6604"
#>  [8] "119e85b7-27cd-4804-8225-318e9c2ef1b3"
#>  [9] "ea8401c7-28a0-47b2-8c2e-70ced749b5f0"
#> [10] "53eea2b9-56c7-4595-92a7-5f2638f1b7a0"
```

``` r
su_flickr <- uuid_to_short(uuids)
su_flickr
#>  [1] "gvLm1LXw61wHBfNUZavhv4" "nh3b1Fz1JjHPnRGrcqSB1M" "7wuHxaKs99QJ3MdNSEgjkZ"
#>  [4] "wzmkYW9SrBCwEh3wLDUKcN" "woU35XF1tjwTEMBXa7ZYN5" "vUqWmG5oHvS8mZKgLEcMe6"
#>  [7] "uiPacp9VtqTaEWwmGnjhh5" "3bc3EyA942nTASZDK1BeKa" "uXC3X1daYcgj3yY5AxRjNC"
#> [10] "bn8vJKrjXXopq7MpxtP9AC"
```

``` r
su_bitcoin <- uuid_to_short(uuids, "bitcoin58")
su_bitcoin
#>  [1] "GWmM1mxX61XicFouzAWHW4" "NH3B1ga1jKipNrhSCRsc1n" "7XViYAkT99qj3nDosfGKLz"
#>  [4] "XaMLyw9sScdXfH3XmeukCo" "XPu35xg1UKXtfncxA7zyo5" "WuRwMh5PiWs8MzkGmfCnE6"
#>  [7] "VJpACQ9vURtAfwXMhNKHH5" "3BC3fZb942Ntbszek1cEkA" "Vxd3x1DAyCGK3Zy5bYrKod"
#> [10] "BN8WjkSKxxPQR7nQYUp9bd"
```

Convert short uuid to uuid (default flickrBase58 alphabet)

``` r
uuids2 <- short_to_uuid(su_flickr)
uuids2
#>  [1] "7da0fa53-e865-4df7-ab8c-593102952d8d"
#>  [2] "ac4d26ae-a444-4b18-bb6b-4af5e7e6c689"
#>  [3] "34d8d78e-d839-4a5d-95a6-701f7364c987"
#>  [4] "f79a3df1-13e7-4f50-b510-65b1ccda6728"
#>  [5] "f624974e-3ec7-4eab-82a7-ec7b259b5a78"
#>  [6] "f22b10b5-4738-401c-a22f-e13191407823"
#>  [7] "e53c96cc-ff7b-4786-8759-df8f66af6604"
#>  [8] "119e85b7-27cd-4804-8225-318e9c2ef1b3"
#>  [9] "ea8401c7-28a0-47b2-8c2e-70ced749b5f0"
#> [10] "53eea2b9-56c7-4595-92a7-5f2638f1b7a0"
```

``` r
uuids3 <- short_to_uuid(su_bitcoin, "bitcoin58")
uuids3
#>  [1] "7da0fa53-e865-4df7-ab8c-593102952d8d"
#>  [2] "ac4d26ae-a444-4b18-bb6b-4af5e7e6c689"
#>  [3] "34d8d78e-d839-4a5d-95a6-701f7364c987"
#>  [4] "f79a3df1-13e7-4f50-b510-65b1ccda6728"
#>  [5] "f624974e-3ec7-4eab-82a7-ec7b259b5a78"
#>  [6] "f22b10b5-4738-401c-a22f-e13191407823"
#>  [7] "e53c96cc-ff7b-4786-8759-df8f66af6604"
#>  [8] "119e85b7-27cd-4804-8225-318e9c2ef1b3"
#>  [9] "ea8401c7-28a0-47b2-8c2e-70ced749b5f0"
#> [10] "53eea2b9-56c7-4595-92a7-5f2638f1b7a0"
```

comparison

``` r
identical(uuids, uuids2)
#> [1] TRUE
identical(uuids, uuids3)
#> [1] TRUE
```
