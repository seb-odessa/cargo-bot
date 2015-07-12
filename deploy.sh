#!/bin/bash

rev=$(git rev-parse --short HEAD)

cd target/doc

git init
git config user.name "seb-odessa"
git config user.email "seb@ukr.net"

git remote add upstream "git@github.com/seb-odessa/cargo-bot"
git fetch upstream && git reset upstream/gh-pages

touch .

git add -A .
git commit -m "rebuild pages at ${rev}"
git push -q upstream HEAD:gh-pages
