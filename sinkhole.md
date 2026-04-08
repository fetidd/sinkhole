### DNS Sinkhole with Analytics
A Pi-hole replacement built in Rust. Intercepts DNS queries, blocks ad/tracker domains, logs everything to Postgres.

**What it does:**
1. **DNS resolver** — sits between your devices and upstream DNS, intercepts lookups
2. **Sinkhole** — returns NXDOMAIN for blocked domains (ads, telemetry, trackers)
3. **Per-device logging** — every query stored in Postgres, queryable by device/time/domain
4. **Analytics** — which devices are noisiest, which hours, which domains; timeseries in Postgres
5. **Per-device rules** — block/allow lists per device, not just globally
6. **API** — temporarily disable blocking, manage rules without touching config files

**Why it has users:** Pi-hole is a Python/bash/PHP sprawl that's painful to maintain. A Rust rewrite is a single binary, minimal memory, thousands of queries/second on a Pi. You also actually understand all of it.

**Start small:** DNS resolver that applies a blocklist and logs queries. Analytics and per-device rules are additive.

> **Windows compatibility:** Significant issues. Windows has a built-in DNS Client service (`Dnscache`) permanently bound to port 53 — you'd have to disable it before this can listen, which breaks normal system DNS resolution in the meantime. There's no clean way to run this alongside normal Windows operation. Designed for Linux/Pi only; don't try to port it.

**TODO:**

*Core DNS*
- [ ] UDP DNS resolver: listen on port 53, parse queries, forward to upstream (e.g. 1.1.1.1)
- [ ] Blocklist loading: parse hosts-file format (Pi-hole compatible) into an in-memory set
- [ ] Sinkhole response: return NXDOMAIN (or 0.0.0.0) for blocked domains
- [ ] Blocklist auto-update: fetch upstream blocklists on a schedule, hot-reload without restart
- [ ] CNAME cloaking detection: resolve CNAMEs and block if the final target is on the blocklist

*Logging*
- [ ] Log every query to Postgres: timestamp, source IP, domain, blocked yes/no, response time
- [ ] Device registry: map IP addresses to friendly names (e.g. `192.168.1.5` → "Ben's phone")
- [ ] Per-device query history: filterable by device, domain, time range, blocked/allowed

*Analytics*
- [ ] Top blocked domains over last 24h / 7d
- [ ] Per-device query volume timeseries (queries per hour)
- [ ] Block rate percentage over time
- [ ] "What is this device talking to?" view per device

*Rules*
- [ ] Global allow list: never block these domains regardless of blocklist
- [ ] Per-device allow/block overrides
- [ ] Temporary disable: allow all queries from a device for N minutes via API (for when an app breaks)
- [ ] Regex-based block rules for pattern matching (e.g. all `*.telemetry.*` subdomains)

*Operational*
- [ ] HTTP API for managing rules, querying logs, triggering blocklist refresh
- [ ] Web dashboard: query log, analytics charts, device list, rule management
- [ ] Config file for upstream DNS servers, listen address, Postgres connection
- [ ] Systemd unit file for running as a daemon on a Pi

**Stack:** Rust + Postgres, runs on Raspberry Pi

---
