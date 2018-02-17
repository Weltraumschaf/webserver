#!/usr/bin/env bash

### The MIT License (MIT)
###
### Copyright (c) 2014 Mathijs van de Nes
###
### Permission is hereby granted, free of charge, to any person obtaining a copy
### of this software and associated documentation files (the "Software"), to deal
### in the Software without restriction, including without limitation the rights
### to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
### copies of the Software, and to permit persons to whom the Software is
### furnished to do so, subject to the following conditions:
###
### The above copyright notice and this permission notice shall be included in all
### copies or substantial portions of the Software.
###
### THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
### IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
### FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
### AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
### LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
### OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
### SOFTWARE.

#
# Script to generate the doc and push it to GH page by Travis CI.
#
# Inspired by Mathijs van de Nes from https://github.com/mvdnes/zip-rs/
#

set -e
set -u

if  [ "${TRAVIS_BRANCH}" = "master" ] &&
    [ "${TRAVIS_PULL_REQUEST}" = "false" ] &&
    [ "{$TRAVIS_REPO_SLUG}" = "weltraumschaf/webserver" ] &&
    [ "{$TRAVIS_RUST_VERSION}" = "stable" ]
then
    echo "Publishing documentation ..."

    echo "TRAVIS_BRANCH: ${TRAVIS_BRANCH}"
    echo "TRAVIS_PULL_REQUEST: ${TRAVIS_PULL_REQUEST}"
    echo "TRAVIS_REPO_SLUG: {$TRAVIS_REPO_SLUG}"
    echo "TRAVIS_RUST_VERSION: {$TRAVIS_RUST_VERSION}"

    pushd "${HOME}"
    git clone --quiet --branch=gh-pages https://${TOKEN}@github.com/weltraumschaf/webserver gh-pages > /dev/null

    pushd gh-pages
    git config user.email "travis@travis-ci.org"
    git config user.name "travis-ci"

    git rm -rf . > /dev/null
    cp -Rf ${TRAVIS_BUILD_DIR}/target/doc/* .

    git reset HEAD -- index.html > /dev/null
    git checkout -- index.html > /dev/null

    git add -f .
    git commit -m "Auto doc upload from travis"
    git push -fq origin gh-pages > /dev/null

    popd
    rm -rfv ./webserver

    popd
    echo "Published documentation :-)"
fi
