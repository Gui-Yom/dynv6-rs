# dynv6-rs

A CLI to interact with the [dynv6](https://dynv6.com) API.

This repository also contains a library you can use to interact with the dynv6 api within your application. It is based
on [ureq](https://lib.rs/ureq), so no async.

Bonus : a tool to fulfill the let's encrypt dns challenge (using `exec` from lego). I use this with my traefik instance
to provision let's encrypt certificate automatically.
