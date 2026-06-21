# JosdorOS

> [!WARNING]  
> This Operating System is in pre-alpha version, use it at your own risk.

JosdorOS is a lightweight routeur operating system based on Debian 13 and written in Rust

## Features : 
### Network Management

- LAN and WAN configuration
- Hostname Management

### Routing
- IPv4 forwarding
- NAT Configuration
- Internet access for LAN clients

### DHCP
- Integrated DHCP server using dnsmasq
- Custom DHCP settings

### API
- REST API built with Axum
- Network configuration endpoint
- Health endpoint
- System informations / health endpoint

## Architecture : 

```text 
josdorOS/
├── backend/
│   ├── routerd/
│       ├── src/
│       │   ├── api/
│       │   │   ├── monitoring/
│       │   │   │   └── mod.rs
│       │   │   ├── network/
│       │   │   │   ├── config.rs
│       │   │   │   └── mod.rs
│       │   │   ├── setup/
│       │   │   │   └── mod.rs
│       │   │   ├── health.rs
│       │   │   └── mod.rs
│       │   ├── nftables/
│       │   │   ├── config.rs
│       │   │   └── mod.rs
│       │   ├── setup/
│       │   │   ├── boot.rs
│       │   │   ├── config.rs
│       │   │   ├── default_config.toml
│       │   │   ├── mod.rs
│       │   │   ├── network.rs
│       │   │   └── utils.rs
│       │   └── main.rs
│       ├── target/
│       ├── Cargo.lock
│       └── Cargo.toml
└── README.md
```

