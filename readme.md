# Learning Rust

## Installing rustup on Linux or macOS

> curl --proto '=https' --tlsv1.2 <https://sh.rustup.rs> -sSf | sh

The command downloads a script and starts the installation of the rustup tool, which installs the latest stable version of Rust.

## Installing rustup on Windows

Go to <https://www.rust-lang.org/tools/install> and follow the instructions for installing Rust. At some point in the installation, you’ll receive a message explaining that you’ll also need the C++ build tools for Visual Studio 2013 or later. The easiest way to acquire the build tools is to install Build Tools for Visual Studio 2019. When asked which workloads to install make sure “C++ build tools” is selected and that the Windows 10 SDK and the English language pack components are included.

## You can simply run a rust program by using

> rustc main.rs

This will create an excutable in the same directory.

## You can install the latest stable version of Rust by using

> rustup

## CARGO

Cargo is Rust’s build system and package manager. Most Rustaceans use this tool to manage their Rust projects because Cargo handles a lot of tasks for you, such as building your code, downloading the libraries your code depends on, and building those libraries. (We call the libraries that your code needs dependencies.)

To check If cargo is intalled or not, use the below command. This Comamnd is also used to check the version of Cargo you're using.

> cargo --version

## Creating a Project with Cargo

To create a Project use below comamnd:

> cargo new project_Name

The first command created a new directory called project_Name . We’ve named our project porject_Name , and Cargo creates its files in a directory of the same name.

Go into the project_Name directory and list the files. You’ll see that Cargo has generated two files and one directory for us: a Cargo.toml file and a src directory with a main.rs file inside.

## Open Cargo.toml in any text editor you want

This file is in the TOML (Tom’s Obvious, Minimal Language) format, which is Cargo’s configuration format.

The first line, [package], is a section heading that indicates that the following statements are configuring a package. 

The next three lines set the configuration information Cargo needs to compile your program: the name, the version, and the edition of Rust to use. 

The last line, [dependencies], is the start of a section for you to list any of your project’s dependencies. In Rust, packages of code are referred to as crates.

## Build a Project

> cargo build

This command creates an executable file in target/debug/project_Name (or target\debug\project_Name.exe on Windows) rather than in your current directory.

## we can also use cargo run to compile the code and then run the resulting executable all in one command:

> cargo run

## Check

Cargo also provides a command called cargo check. This command quickly checks your code to make sure it compiles but doesn’t produce an executable

> cargo check

## Building for Release

> cargo build --release

you can use cargo build --release to compile it with optimizations. This command will create an executable in target/release instead of target/debug. The optimizations make your Rust code run faster, but turning them on lengthens the time it takes for your program to compile.
