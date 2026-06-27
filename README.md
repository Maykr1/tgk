<div align="center">

```
 _____ ____ _  __
|_   _/ ___| |/ /
  | || |  _| ' / 
  | || |_| | . \ 
  |_| \____|_|\_\

The Greatest Knight
```

**Your data is out there. TGK finds it. TGK kills it.**

*A local-first, zero-trust personal data exposure auditor - built in Rust, run by you, seen by no one else.*

[![Build Status](https://img.shields.io/github/actions/workflow/status/yourusername/tkg/ci.yml?style=flat-square&color=4a9eff)](https://github.com/yourusername/tkg/actions)
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg?style=flat-square)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.78%2B-orange?style=flat-square&logo=rust)](https://www.rust-lang.org/)
[![Platform](https://img.shields.io/badge/Platform-Linux%20%7C%20macOS%20%7C%20Windows-blue?style=flat-square)]()

</div>

---

## The Problem

Your data is everywhere, and data breaches are common.

Your phone number is on Whitepages. Your address history is on Spokeo. Your full name, age, and relatives are on BeenVerified. Your email showed up in a data breach three years ago and has been in a database ever since. 

You didn't put it there, or you may have not known it was even there in the first place.

Companies like Incogni, DeleteMe, and Aura will happily remove it for you.

All you have to do is hand them your name, address, phone number, email address, date of birth, and a paid subscription. To remove your data from data brokers, you have to give your data to a data broker.

Data and personal privacy shouldn't require a computer science degree, a lawyer, or a subscription to yet another service to protect yourself.

---

**TGK doesn't work that way.**

Your profile never leaves your machine. Ever. TGK runs locally, queries public sources on your behalf, generates removal requests, tracks their status, and then gets out of your way. No account setup. No subscriptions. No telemetry. No data breaches that expose the personal information you paid to have protected.