# Epism

## Introduction

Epism (**E**xplicitely **P**arallel **I**nstruction **S**et **M**achine) is my session project for my virtual machines course at UQÀM winter 2022. It is composed of two parts: EpismISA, an explicitely parallel instrution set, and EpismVM, a virtual machine to emulate it.

## Architecture

EpismISA is an explicitely parallel instruction set architecture. It aims to make it provide an instruction set that allows to explicitely programs that can use a lot of instruction-level parallelism without requiring any runtime analysis.

## Machine

EpismVM is a virtual machine to run programs written in EpismISA. It runs synchronously but aims to emulate the parallelism of EpismISA by measuring the theorical performance improvements that would have happened if the code was indeed run in parallel.
