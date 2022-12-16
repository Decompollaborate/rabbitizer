#!/bin/bash

# SPDX-FileCopyrightText: © 2022 Decompollaborate
# SPDX-License-Identifier: MIT

set -e

INPUT_FILE=$1
OUTPUT_FILE=$2
HEADER_GUARD_BAD=$3

# Change dots to underscores
HEADER_GUARD=$(echo ${HEADER_GUARD_BAD} | sed 's/\./_/g')

echo "#ifndef ${HEADER_GUARD}_automatic" > ${OUTPUT_FILE}
echo "#define ${HEADER_GUARD}_automatic" >> ${OUTPUT_FILE}

echo >> ${OUTPUT_FILE}
echo "/* Automatically generated. DO NOT MODIFY */" >> ${OUTPUT_FILE}
echo >> ${OUTPUT_FILE}

cpp -P ${INPUT_FILE} >> ${OUTPUT_FILE}

echo >> ${OUTPUT_FILE}

echo "#endif" >> ${OUTPUT_FILE}
