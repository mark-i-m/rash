# Rash

Rash stands for Remote Access SHell... don't ask me; I didn't pick the name...
(Actually I did, but still don't ask)

It is a quick and dirty utility for running commands on a remote machine via SSH with public key authorization.

The objective is that if you are running a bunch of long-running benchmarks on a remote machine and measuring system metrics, you may want to write a script to run all of these benchmarks automatically. However, the script itself has some overhead beyond your SSH connection, which may be undesirable. I ran into this problem when measuring aspects of kernel memory mapping subsystems. Using this utility, you only have the overhead of the SSH connection, which is about as minimal as I could make it.

There are some additional benefits: since the script is running locally, it can survive restarts/reconfiguration of the remote system. In particular, rash supports two metacommands, `sleep` and `reconnect`, which cause rash to sleep for a minute or attempt to reconnect to the remote, respectively. These are usefull if your script needs to reboot the remote. See the `test.sh` script for examples.

## Usage

The utility is written in Rust. You can install rust (including cargo) from [https://www.rust-lang.org/en-US/install.html](https://www.rust-lang.org/en-US/install.html).

To build

```sh
cargo build --release
```

To run
```sh
cargo run --release <remote host> <remote user> <path to local script to run remotely>
```

Note that you need to be able to log in to the remote as the user via public key over SSH.
