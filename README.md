# dist-logger

`dist-logger` is a simple program designed to perform the following actions:

1. Create containers.
2. Assign neighbors to each container.
3. Log when containers start executing.
4. Make containers greet their neighbors.
5. Log which container greeted whom, and when, in the log.

## Requirements

Before getting started, make sure you have the following dependencies installed:

- [Rust](https://www.rust-lang.org/) (with cargo).
- [Protocol buffers compiler (`protoc`)](https://protobuf.dev/downloads/) (required by [tonic](https://github.com/hyperium/tonic))
- [Incus](https://linuxcontainers.org/incus/)
- [Ansible](https://docs.ansible.com/)
- [Bash](https://www.gnu.org/software/bash/)

## Getting Started

To get `dist-logger` up and runnig, follow these steps:

1. Compile the program (in either release of debug) with `cargo`.
2. Run the [`setup.sh`](https://github.com/thewillyan/dist-logger/blob/main/scripts/setup.sh) script to setup the Incus project and the necessary containers.
3. Run the ansible [`playbook.yml`](https://github.com/thewillyan/dist-logger/blob/main/ansible/playbook.yml) with the `ansible-playbook` command.

Here is a example of how to run the program in debug mode:

```shell
cargo build
cd dist-logger
scripts/setup.sh
cd ansible && ansible-playbook playbook.yml --tags debug
```

To verify the logs of each container run the [`show_logs.sh`](https://github.com/thewillyan/dist-logger/blob/main/scripts/show_logs.sh) script.

### Available scripts

In addition to `setup.sh` and `show_logs.sh`,

the `scripts` directory contains several other useful scripts for different tasks. You can explore the full list of available scripts [here](https://github.com/thewillyan/dist-logger/tree/main/scripts).
