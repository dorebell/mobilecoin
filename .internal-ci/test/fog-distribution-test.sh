#!/bin/bash
# Copyright (c) 2018-2022 The MobileCoin Foundation
#
# Wrapper around fog-distribution binary to add some simple checks and clean defaults.
#

set -e

run_file=/var/tmp/.fog-distribution-already-ran

if [ -f "${run_file}" ]
then
    echo "-- Cowardly refusing to run fog-distribution a second time."
    exit 0
fi

usage()
{
    echo "Usage --domain"
    echo "    --domain - base domain for consensus services"
}

while (( "$#" ))
do
    case "${1}" in
        --help | -h)
            usage
            exit 0
            ;;
        --domain )
            domain="${2}"
            shift 2
            ;;
        *)
            echo "${1} unknown option"
            usage
            exit 1
            ;;
    esac
done


touch "${run_file}"

fog-distribution --sample-data-dir /tmp/sample_data \
    --peer "mc://node1-${NAMESPACE}.${domain}:443" \
    --peer "mc://node2-${NAMESPACE}.${domain}:443" \
    --peer "mc://node3.${NAMESPACE}.${domain}:443" \
    --num-tx-to-send 20


# assumes
# /tmp/sample_data/keys - path to init keys where funds are coming from
# /tmp/sample_data/fog_keys - path to destination keys
