Neutral IPC server
==================

A server and different clients for different programming languages are provided for [Neutral TS](https://github.com/FranBarInstance/neutralts)

IPC Server
----------

Currently the IPC server is under development, you can use it but you must take into account that the stable version could present incompatibilities, for example in the configuration.

For a peronalized configuration modify neutral-ipc-cfg.json and put it in the /etc directory, this is the default configuration:

```
{
    "host": "127.0.0.1",
    "port": "4273"
}
```

Navigate to the ipc directory and:

```
cargo run --release
```

You can place the generated binary wherever you want.

Debian
------

Create your own deb package, navigate to the ipc directory and:

```
cargo deb
```

You will find the *.deb in target/debian

IPC Client
----------

There are different clients, it is implemented as a class, place it and act according to your framework and programming language.

It is accompanied by a class with the configuration that you can change according to the configuration of your IPC server.

[more info](https://github.com/FranBarInstance/neutral-ipc)
