#' Generate new Short UUID
#'
#' @param n the number of short uuids to generate
#' @name short_uuid
#' @export
short_uuid <- function(uuid, short = c("flickr_base58", "bitcoin58")) {
  short <- match.arg(short)
  switch(
    short,
    "flickr_base58" = short_flickr_base58_(uuid),
    "bitcoin58" = short_bitcoin58_(uuid)
  )
}


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
