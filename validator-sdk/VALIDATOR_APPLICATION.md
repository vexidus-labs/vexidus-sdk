# Vexidus Beta Validator — Application

We are onboarding a first cohort of **up to ~20 beta-testnet validators**, with more
slots opening in later waves. Onboarding is **coordinated and application-only** — the
network is completing a consensus-core upgrade, so a node built from the public setup
steps cannot yet auto-join. **Accepted operators receive the correct binary, genesis
snapshot, and current bootstrap peers directly.**

Selection favors **geographic diversity** (we balance the validator set across regions
for low-latency consensus), demonstrated **operational discipline**, and a **dedicated**
server that meets the requirements below.

## How to apply

**Fastest — online form:**
**https://docs.google.com/forms/d/e/1FAIpQLSczUhY4F_eNL_buvMphFMPFwGkth3lxtMwB7ynirgBXX6-kCA/viewform**

**Or email** the answers below to **validators@vexidus.com** (subject:
`Beta Validator Application — <your name/org>`). Fill in every field marked ★.

## Requirements

> One **dedicated** machine per validator. Do **not** co-host other heavy workloads
> (build servers, other chains, AI/inference); resource contention causes missed blocks
> and is grounds for removal from the set.

| Resource | Minimum (beta) | Recommended |
|----------|----------------|-------------|
| CPU | 6 cores, dedicated | 8+ cores |
| RAM | 16 GB | 32 GB |
| Storage | 200 GB **NVMe SSD** | 500 GB NVMe SSD |
| Network | Static public IPv4, 100 Mbps+, low latency | 1 Gbps, low latency |
| OS | Ubuntu 22.04+ LTS (x86_64) | Ubuntu 24.04 LTS |

**Required open port:** `9946/UDP` (QUIC P2P) inbound. RPC (`9933/TCP`) must be
firewalled to localhost / trusted IPs only.

## Application form

*(★ = required.)*

### A. Applicant
1. ★ Name / organization
2. ★ Contact email
3. ★ Real-time chat handle (Telegram or Discord) — used for validator coordination
4. ★ Country + city, and your timezone (UTC offset)
5. Individual operator or organization? (if org, one line on who you are)

### B. Experience
6. ★ Have you operated blockchain validators or full nodes before? Which networks?
7. ★ Comfort with Linux server administration (SSH, systemd, firewall): None / Some / Proficient / Expert
8. Why do you want to run a Vexidus validator? (2–3 sentences)

### C. Server / infrastructure
9. ★ Hosting type & provider (e.g., Hetzner bare-metal, OVH VPS, AWS EC2)
10. ★ Datacenter location (city / country) — we balance the set for geographic + latency diversity
11. ★ CPU — cores + model
12. ★ RAM (GB)
13. ★ Disk — type + capacity (NVMe/SSD required)
14. ★ Dedicated static public IPv4? (yes / no)
15. Network bandwidth / monthly data allowance
16. OS + version

### D. Operations & commitment
17. ★ Can you commit to ~24/7 uptime for the beta period? (yes / no)
18. ★ Can you respond to coordinated upgrade/restart windows on short notice (often <24h)? What hours (and timezone) are you reachable?
19. ★ How will you secure the validator signing key and server access? (e.g., key-only SSH, firewall, encrypted key backup)
20. ★ Acknowledge: this is a **beta testnet** — tokens have no monetary value, and the chain may undergo coordinated fresh-genesis resets. (yes / acknowledged)

---

*Questions? Email **validators@vexidus.com**.*
