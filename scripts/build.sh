#!/bin/bash

go build

# Packages the init files from /cmd/init
pkger -o cmd

# Temporary pkger fix (see https://github.com/markbates/pkger/issues/56)
perl -i -pe 's/.*/package cmd/ if $.==1' cmd/pkged.go
