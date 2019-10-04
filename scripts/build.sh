#!/usr/bin/env bash

docker run --rm -v "$PWD":/home/gradle/project -w /home/gradle/project gradle gradle run
sudo rm -rf build .gradle
sudo chown `whoami`:`whoami` kafka.bnf
