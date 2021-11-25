#!/bin/bash
git submodule update --init --recursive
docker build -t magnesie-photogrammetry .