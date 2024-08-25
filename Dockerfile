# syntax=docker/dockerfile:1.9

FROM docker.gosh.sh/rust AS rust

COPY --link . ./