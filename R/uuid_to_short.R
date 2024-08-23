#' Convert uuid to short
#'
#' @param uuid vector of uuids
#' @export
uuid_to_short <- function(uuid, short = c("flickr_base58", "bitcoin58")) {
  short <- match.arg(short)
  switch(
    short,
    "flickr_base58" = uuid_flickr_to_short_(uuid),
    "bitcoin58" = uuid_bitcoin58_to_short_(uuid)
  )
}
