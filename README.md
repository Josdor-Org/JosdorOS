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

## How to use : 

### Setup VMs : 

I recommend using a software like Proxmox but if you don't have a server at home, you can try using virtualbox or vmware.

- Create a 1st vm with two network interface, one who is connected to your computer network and a another one who is not connected to your network ( a virtual interface ) and boot on JosdorOS ISO
<img width="957" height="492" alt="image" src="https://github.com/user-attachments/assets/c22ab71e-91ba-4676-b1ef-2a2385fb35a9" />
<img width="968" height="539" alt="image" src="https://github.com/user-attachments/assets/e9ffa427-2367-4ed7-b69d-232edc67904b" />


- Create a 2nd vm with only the virtual interface ( it should not connect to your network ) i recommend using a linux distro

<img width="965" height="539" alt="image" src="https://github.com/user-attachments/assets/382e2c5e-b1b7-4c96-a312-bb662e2edddc" />

- start the 1st vm and boot on the iso file. When booted, just type "ip a" command to get the interfaces. The 1st interface shown should have an ip adress, take it in note. Also take in note of the name of the two interfaces ( ex : ens18 and ens19 )

<img width="1109" height="581" alt="image" src="https://github.com/user-attachments/assets/1095aefe-0e63-49e9-84c7-f8fdd7b673d1" />


- Create a 2nd vm with only the virtual interface ( it should not connect to your network ) i recommend using a linux distro
- start the 1st vm and boot on the iso file. When booted, just type "ip a" command to get the interfaces. The 1st interface shown should have an ip adress, take it in note. Also take in note of the name of the two interfaces ( ex : ens18 and ens19 )


### JosdorOS quick setup : 

I recommend using Postman for the next step

- Send a POST request to <machine-ip>/api/setup ( replace <machine-ip> by the IP you noted previously )
- Send the request with this Body ( JSON ) :

```json
{
  "hostname": "JosdorOS",
  "wan_interface": "ens18",  // Replace by the 1st interface
  "lan_interfaces": ["ens19"], // Replace by the 2nd interface
  "dhcp_ip_range_start": "10.10.0.100",
  "dhcp_ip_range_end": "10.10.0.150",
  "dhcp_forwarding_ip": "10.10.0.1",
  "dhcp_lease": "12h"
}
```

- If everything OK you will get this response : 'Network setup completed successfully' if not double-check the parameters.


<img width="1956" height="1374" alt="image" src="https://github.com/user-attachments/assets/f02b3dad-7782-43be-8abd-239e8502005c" />


### Try if everything working

- you can start the 2nd vm and type command "ip a" again and you will normally see 1 interface with an ip, if this interface doesn't have an ip try 'sudo systemctl restart NetworkManager" or "dhclient" it will ask for a new ip to the DHCP server.
- Now, you get a Lan IPv4 and you can try to ping the gateway : "ping 10.10.0.1" or directly a domain name : "ping google.com"

<img width="1142" height="565" alt="image" src="https://github.com/user-attachments/assets/befcb56c-e2ed-4154-937f-ec3dcd283179" />

### Try if everything working

- you can start the 2nd vm and type command "ip a" again and you will normally see 1 interface with an ip, if this interface doesn't have an ip try 'sudo systemctl restart NetworkManager" or "dhclient" it will ask for a new ip to the DHCP server.
- Now, you get a Lan IPv4 and you can try to ping the gateway : "ping 10.10.0.1" or directly a domain name : "ping google.com"

