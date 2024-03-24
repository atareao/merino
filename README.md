```
                     _
 _ __ ___   ___ _ __(_)_ __   ___
| '_ ` _ \ / _ \ '__| | '_ \ / _ \
| | | | | |  __/ |  | | | | | (_) |
|_| |_| |_|\___|_|  |_|_| |_|\___/
```

**A `SOCKS5` Proxy server written in Rust**

[![Crates.io](https://img.shields.io/crates/v/merino.svg)](https://crates.io/crates/merino)
[![stego](https://docs.rs/merino/badge.svg)](https://docs.rs/merino)
[![License](https://img.shields.io/crates/l/pbr.svg)](https://github.com/ajmwagar/merino/blob/master/LICENSE.md)
[![Build Status](https://travis-ci.org/ajmwagar/merino.svg?branch=master)](https://travis-ci.org/ajmwagar/merino)
[![dependency status](https://deps.rs/repo/github/ajmwagar/merino/status.svg)](https://deps.rs/repo/github/ajmwagar/merino)

## 🎁 Features

- Written in **100% Safe Rust**
- Multi-threaded connection handler
- Lightweight (Less than 0.6% CPU usage while surfing the web/streaming YouTube)
- Standalone binary (no system dependencies)
- `1+ Gb/second` connection speeds (**On Gigabit LAN network over ethernet. Results may vary!**)
- Tunable logging (by flags or `RUST_LOG` environmental variable)
- `SOCKS5` Compatible Authentication methods:
  - `NoAuth`
  - Username & Password

## 📦 Installation & 🏃 Usage

### Installation

```bash
docker image pull atareao/merino:latest
```

### Configuration

Make this config file `config.yml`,

```
users:
  - username: username
    password: password
    active: true
  - username: username1
    password: password1
    active: true
  - username: username2
    password: password2
    active: false
fqdns:
  - addr: www.google.es
    active: true
  - addr: www.google.com
    active: true
```

If you don't want to restrict acces to any fqdn, simple `fqdns: []`

### Usage

```bash
docker compose up -d
```
