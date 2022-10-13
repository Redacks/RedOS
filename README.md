# RedOS

## Introduction

I started this project just for fun. In my past i already started writing a kernel from near skrach with C++ but eventually stopped working on it. This time i wanted to try it with RUST and hope that i will get it to do more than my C++ project.

So have fun exploring this project

## Getting started

You will need to have Rust installed:
https://www.rust-lang.org/tools/install

Also you will need to install some kind of emulator or VM to start the ISO file.
In my case I used QEMU:
https://www.qemu.org/download/

For development i used VS-Code combined with the rust-analyzer, but feel free to use any other IDE or texteditor

`cargo install bootimage`

### On linux

`sudo apt install gcc-multilib`
`rustup override set nightly-2022-10-07-x86_64-unknown-linux-gnu`
`rustup component add rust-src --toolchain nightly-2022-10-07-x86_64-unknown-linux-gnu`
`rustup component add llvm-tools-preview `

## Running it

### Running

You can run the current kernel with `cargo run`

## What can it do

Currently the kernel can't do that much. It can only print text to the screen and get some keyboard inputs.

## Working on it

Currently i am adding more interrupts and the gdt.
After that i want to add paging and keyboard inputs.

https://github.com/phil-opp/blog_os/blob/post-07/src/interrupts.rs
