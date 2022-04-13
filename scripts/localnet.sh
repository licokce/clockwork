#!/bin/bash

solana-test-validator -r \
    --bpf-program target/deploy/cronos_heartbeat-keypair.json target/deploy/cronos_heartbeat.so \
    --bpf-program target/deploy/cronos_pool-keypair.json target/deploy/cronos_pool.so \
    --bpf-program target/deploy/cronos_scheduler-keypair.json target/deploy/cronos_scheduler.so \
    --geyser-plugin-config plugin/config.json