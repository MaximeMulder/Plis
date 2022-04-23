# Plis

## Introduction

Plis is my session project for my virtual machines course at UQÀM winter 2022. It is composed of two parts: PlisISA, an explicitely parallel instrution set, and PlisVM, a virtual machine to emulate it.

## Architecture

PlisISA is an explicitely parallel instruction set architecture. It aims to make it provide an instruction set that allows to explicitely programs that can use a lot of instruction-level parallelism without requiring any runtime analysis.

## Machine

PlisVM is a virtual machine to run programs written in PlisISA. It runs synchronously but aims to emulate the parallelism of PlisISA by measuring the theorical performance improvements that would have happened if the code was indeed run in parallel.
