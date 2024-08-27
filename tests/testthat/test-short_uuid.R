UUID <- "bf34000c-3779-4519-a82f-8c8b45b80dab"

test_that("check uuid short", {
  flickr <- new_short(1)
  bitcoin <- new_short(1, "bitcoin58")
  expect_equal(nchar(flickr), 22)
  expect_equal(nchar(bitcoin), 22)
  expect_true(all(grepl("[0-9a-zA-Z]{22}", c(flickr, bitcoin))))
})

test_that("check flickr encoding", {
  expect_equal(uuid_to_short(UUID) , "pBpYVoXrShgRxX5o3yg6Kz")
})

test_that("check bitcoin encoding", {
  expect_equal(uuid_to_short(UUID, "bitcoin58") , "QcQyvPxSsHGrYx5P3ZG6ka")
})

test_that("check flickr roundhouse", {
  short_uuid <- uuid_to_short(UUID)
  expect_equal(short_to_uuid(short_uuid) , UUID)
})

test_that("check bitcoin58 roundhouse", {
  short_uuid <- uuid_to_short(UUID, "bitcoin58")
  expect_equal(short_to_uuid(short_uuid, "bitcoin58") , UUID)
})

