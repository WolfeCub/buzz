#!/usr/bin/env bash

curl -s -o /dev/null localhost:8080/foo
curl -s -o /dev/null localhost:8080/params/hello
curl -s -o /dev/null localhost:8080/query?test=blah
curl -s -o /dev/null localhost:8080/inject
curl -s -o /dev/null localhost:8080/json -X POST --data '{"foo": "hello", "bar": 42}'
curl -s -o /dev/null localhost:8080/cast/42
