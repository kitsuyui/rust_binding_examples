#!/usr/bin/env bash
set -eu

BASEDIR=$PWD
for package in $PWD/../target/wheels/*.whl
do
    if [[ $package != *"$(arch)"* ]]; then
        continue
    fi
    test_dir=$(mktemp -d)
    cd $test_dir
        python3 -m venv .venv
            ./.venv/bin/pip install $package
            ./.venv/bin/python3 -c 'from amazing_calc import my_calc; assert my_calc(1, 2, 3) == "9"; print("OK")'
        rm -rf .venv
    cd "$BASEDIR"
    rm -rf $test_dir
done
