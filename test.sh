#!/usr/bin/env bash

curl localhost:8080/foo
curl localhost:8080/params/hello
curl localhost:8080/query?test=blah
curl localhost:8080/inject
curl localhost:8080/json -X POST --data '{"foo": "hello", "bar": 42}'
curl localhost:8080/cast/42
