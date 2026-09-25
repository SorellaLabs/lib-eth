# lib-reth

Unified interface for connecting to and interacting with Reth nodes.

## Overview

This crate provides a comprehensive library for connecting to Ethereum endpoints using Reth, supporting both Ethereum mainnet and Base.

## Connection Types

- HTTP (default)
- IPC (feature = `ipc`)
- WebSocket (feature = `ws`)
- Direct database access via libmdbx (feature = `reth-db`)

## Features

- `full` - All connection types and integrations
- `revm` - REVM execution support
- `base-reth-db` - Base node support
- `rayon` - Parallel execution support

## Supported Functionality

- RPC client implementations
- Streaming support for blocks, transactions, and logs
- Direct database access for local nodes
- Integration with Uniswap storage utilities