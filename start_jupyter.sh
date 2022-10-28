#!/bin/bash

echo " ================ PATH=${PATH}"
echo " ================ PYTHONPATH=${PYTHONPATH}"

jupyter notebook \
    --ip=0.0.0.0 \
    --no-browser \
    --allow-root \
    --NotebookApp.token='' \
    --NotebookApp.password=''
