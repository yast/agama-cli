# How to set up a backend for testing this CLI frontend

I needed a testing instance of the D-Installer backend so that this
command-line frontend has something to talk to.

## Summary

1. Take the container used for continuous integration (CI) testing of the
   backend
2. Give it a git checkout of the backend
3. Install the backend within the container
4. Copy the frontend binary into the container

## Considered Alternatives

My first plan had a different finale, 4. Make the D-Bus service visible
ouside the container, but I hit an issue with D-Bus authentication, hopefully
solvable.

Josef wanted to test against a different container (...) but that one was a
bit old and the D-Bus API was mismatched between frontend and backend.

## Details

WIP. Basically pick the useful bits from the `integration-tests` part of https://github.com/yast/d-installer/blob/25462f57ab695d6910beb59ff0b21a7afaeda47e/.github/workflows/ci.yml

