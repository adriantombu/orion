#!/bin/bash

rm cmd/pkged.go
pkger -o cmd

go build
