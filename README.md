# MagiChat

MagiChat allows you to communicate with any computer in your network via UDP.
MagiChat is heavily inspired to Nintendo's Pictochat (I had so much fun with all the DS's ecosystem, I wanted
to pay homage).
Now as a TUI thanks to [Ratatui](https://ratatui.rs/)!

# Usage

Before using Pictochat you need to have installed [cargo](https://doc.rust-lang.org/cargo/).
Run the following commands:

```bash
git clone git@github.com:MagiHotline/pictochat.git
cd pictochat
cargo run
```

You will be asked to set three things:

- Your host name
- In which socket you want to bind yourself (i.e. 192.168.0.2:8888)
- To which socket you want to connect

Once you required the data, press `[Enter]` and you will start the chat if all went correctly.

> [!WARNING]
> You can connect only to who is already inside your LAN. Not from outside since Pictochat does not have port
> forwarding natively.

## Roadmap

- [x] make it work
- [x] TUI
- [ ] Search for active connections in the LAN
- [ ] Making it more similar to Pictochat, so the user and the connected people can draw together in the TUI
