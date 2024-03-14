#!/bin/bash

python3 -m venv .venv
source .venv/bin/activate
pip install componentize-py==0.11.0 spin-sdk==2.0.0rc2 mypy==1.8.0
componentize-py -w spin-http componentize app -o app.wasm