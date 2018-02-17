#!/usr/bin/env bash

set -e
set -u

echo "Publishing documentation ..."

cargo clean
cargo build
#cargo test
cargo doc --no-deps
rustdoc --test README.md -L target

tmp_dir=$(mktemp -d)

pushd "${tmp_dir}"
git clone --quiet --branch=gh-pages git@github.com:Weltraumschaf/webserver.git gh-pages

pushd gh-pages
git config user.email "travis@travis-ci.org"
git config user.name "travis-ci"

git rm -rf . > /dev/null
cp -Rf ${PROJECT_HOME}/target/doc/* .

git reset HEAD -- index.html > /dev/null
git checkout -- index.html > /dev/null

git add -f .
git commit -m "Auto doc upload from travis"
git push -fq origin gh-pages > /dev/null

popd
popd
rm -rfv "${tmp_dir}"

echo "Published documentation :-)"