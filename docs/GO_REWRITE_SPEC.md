# OPNsense Config Faker — Go Rewrite Specification

This document captures everything needed to rewrite `opnsense-config-faker` (Rust) in Go, sharing OPNsense schema models with `opnDossier`.

## 1. Project Purpose

Generate realistic, valid OPNsense `config.xml` files with faked data for testing, training, and development. The tool produces complete OPNsense configuration files that comply with the `opnsense-config.xsd` schema.

**Primary use case**: `opnsense-config-faker xml --base-config config.xml --count 25` → generates 25 unique config.xml files with realistic VLANs, DHCP, firewall rules, NAT, and VPN configurations injected into a base OPNsense template.

---

## 2. Shared Schema Strategy with opnDossier

### 2.1 opnDossier Already Defines These Types

The `github.com/EvilBit-Labs/opnDossier/internal/schema` package already has production-grade OPNsense schema types. The Go rewrite **must import and use these directly** rather than duplicating them.

| opnDossier Type                                             | Package  | Used By Faker For                                 |
| ----------------------------------------------------------- | -------- | ------------------------------------------------- |
| `OpnSenseDocument`                                          | `schema` | Root config.xml structure                         |
| `Interfaces` (map-based)                                    | `schema` | Generated interface entries (opt0, opt1, ...)     |
| `Interface`                                                 | `schema` | Individual interface config (IP, subnet, gateway) |
| `VLANs` / `VLAN`                                            | `schema` | Generated VLAN definitions                        |
| `Filter` / `Rule`                                           | `schema` | Generated firewall rules                          |
| `Source` / `Destination`                                    | `schema` | Firewall rule endpoints                           |
| `Nat` / `NATRule` / `InboundRule`                           | `schema` | Generated NAT mappings                            |
| `Outbound`                                                  | `schema` | Outbound NAT rules                                |
| `Dhcpd` (map-based) / `DhcpdInterface`                      | `schema` | Generated DHCP server configs                     |
| `DHCPStaticLease`                                           | `schema` | Static DHCP reservations                          |
| `Range`                                                     | `schema` | DHCP address ranges                               |
| `OpenVPN` / `OpenVPNServer` / `OpenVPNClient`               | `schema` | Generated OpenVPN configs                         |
| `WireGuard` / `WireGuardServerItem` / `WireGuardClientItem` | `schema` | Generated WireGuard configs                       |
| `IPsec` / `Swanctl`                                         | `schema` | Generated IPsec configs                           |
| `Gateways` / `Gateway` / `GatewayGroup`                     | `schema` | Generated gateway definitions                     |
| `StaticRoutes` / `StaticRoute`                              | `schema` | Generated static routes                           |
| `System`                                                    | `schema` | System config (hostname, domain, etc.)            |
| `BoolFlag`                                                  | `schema` | XML boolean marshaling helper                     |
| `InterfaceList`                                             | `schema` | Comma-separated interface lists                   |
| `Created` / `Updated`                                       | `schema` | Audit timestamps                                  |
| `Cert` / `CertificateAuthority`                             | `schema` | Certificate generation                            |

### 2.2 Types the Faker Must Define Itself

These are faker-specific types that have no equivalent in opnDossier:

| Type                 | Purpose                                                                     |
| -------------------- | --------------------------------------------------------------------------- |
| `VlanConfig`         | Internal generation model: VLAN ID + network + description + WAN assignment |
| `DhcpServerConfig`   | Computed DHCP config: ranges, lease times, DNS, NTP, static reservations    |
| `StaticReservation`  | MAC → IP hostname mapping for DHCP                                          |
| `FirewallRuleSet`    | Collection of rules for a VLAN at a given complexity level                  |
| `FirewallComplexity` | Enum: Basic (3 rules), Intermediate (7), Advanced (15)                      |
| `NatMapping`         | Internal NAT generation model with rule type enum                           |
| `NatRuleType`        | Enum: PortForward, SNAT, DNAT, OneToOne, Outbound                           |
| `VpnConfig`          | Internal VPN generation model                                               |
| `VpnType`            | Enum: OpenVPN, WireGuard, IPSec                                             |
| `WanAssignment`      | Strategy enum: Single, Multi, Balanced                                      |
| `PerformanceMetrics` | Generation timing and memory tracking                                       |

### 2.3 opnDossier Schema Package Extraction

To share schemas, one of these approaches is needed:

**Option A — Import directly**: The faker imports `github.com/EvilBit-Labs/opnDossier/internal/schema`. This requires either:

- Making `internal/schema` a public package (move to `pkg/schema`), or
- Both projects living in the same Go module/workspace

**Option B — Extract shared module**: Create `github.com/EvilBit-Labs/opnsense-schema` as a shared Go module that both projects import.

**Option C — Go workspace**: Use `go.work` to link both projects locally, with `replace` directives for the schema package.

---

## 3. Architecture — Rust to Go Mapping

### 3.1 Module Structure

```
Rust (current)                    Go (target)
─────────────────────────────────────────────────────────
src/cli/                    →     cmd/                        (Cobra commands)
src/cli/commands/generate.rs →    cmd/generate.go
src/cli/commands/validate.rs →    cmd/validate.go
src/cli/commands/completions →    cmd/completion.go
src/generator/              →     internal/generator/
src/generator/vlan.rs       →     internal/generator/vlan.go
src/generator/firewall.rs   →     internal/generator/firewall.go
src/generator/nat.rs        →     internal/generator/nat.go
src/generator/vpn.rs        →     internal/generator/vpn.go
src/generator/departments.rs →    internal/generator/departments.go
src/generator/performance.rs →    internal/generator/performance.go
src/model/                  →     (use opnDossier/internal/schema)
src/model/error.rs          →     internal/errors/errors.go
src/validate/               →     internal/validate/
src/xml/                    →     internal/xmlgen/
src/xml/engine.rs           →     internal/xmlgen/engine.go
src/xml/builder.rs          →     internal/xmlgen/builder.go
src/xml/template.rs         →     internal/xmlgen/template.go
src/xml/injection.rs        →     internal/xmlgen/injection.go
src/xml/streaming.rs        →     internal/xmlgen/streaming.go
src/io/csv.rs               →     internal/csvio/csvio.go
src/utils/rfc1918.rs        →     internal/netutil/rfc1918.go
```

### 3.2 Dependency Mapping

| Rust Crate             | Go Equivalent                                                                                     |
| ---------------------- | ------------------------------------------------------------------------------------------------- |
| `clap` (CLI)           | `cobra` + `pflag` (already used by opnDossier)                                                    |
| `serde` / `serde_json` | `encoding/xml`, `encoding/json` (stdlib)                                                          |
| `quick-xml`            | `encoding/xml` (stdlib)                                                                           |
| `csv`                  | `encoding/csv` (stdlib)                                                                           |
| `rand` / `rand_chacha` | `math/rand/v2` (stdlib, seedable)                                                                 |
| `fake`                 | `github.com/brianvoe/gofakeit/v7` or custom                                                       |
| `uuid`                 | `github.com/google/uuid`                                                                          |
| `ipnet` / `ipnetwork`  | `net/netip` (stdlib, Go 1.18+)                                                                    |
| `thiserror` / `anyhow` | `errors` + `fmt.Errorf` with `%w`                                                                 |
| `indicatif`            | `github.com/schollz/progressbar/v3` or `github.com/charmbracelet/bubbles` (opnDossier uses Charm) |
| `console`              | `github.com/charmbracelet/lipgloss` (already in opnDossier)                                       |
| `bumpalo` (arena)      | `sync.Pool` or arena patterns                                                                     |
| `lru`                  | `github.com/hashicorp/golang-lru/v2`                                                              |
| `smallvec`             | slices (Go handles this well natively)                                                            |
| `rustc-hash` (FxHash)  | `github.com/dolthub/maphash` or stdlib `hash/maphash`                                             |
| `criterion`            | `testing.B` (stdlib benchmarks)                                                                   |
| `proptest`             | `testing/quick` or `github.com/flyingmutant/rapid`                                                |
| `insta`                | `github.com/bradleyjkemp/cupaloy` or `github.com/sebdah/goldie`                                   |
| `rayon`                | goroutines + `sync.WaitGroup` or `errgroup`                                                       |

---

## 4. Generator Logic — Complete Specification

### 4.1 VLAN Generator

**Purpose**: Generate unique, RFC-1918 compliant VLAN configurations.

**Constraints**:

- VLAN ID range: 10–4094 (max 4085 unique)
- Networks: RFC 1918 only (10.0.0.0/8, 172.16.0.0/12, 192.168.0.0/16)
- Default subnet: /24
- All VLAN IDs must be unique within a generation run
- All IP networks must be unique within a generation run

**VlanConfig** (faker-internal model):

```go
type VlanConfig struct {
    VlanID        uint16   // 10-4094
    IPNetwork     string   // CIDR notation, e.g. "10.42.7.0/24"
    Description   string   // "{Department} VLAN {ID}"
    WanAssignment uint8    // 1-3
}
```

**Derived values** (computed from VlanConfig, not stored):

| Value               | Derivation                                               |
| ------------------- | -------------------------------------------------------- |
| Gateway IP          | Network + `.1` (e.g., `10.42.7.1`)                       |
| DHCP range start    | Network + `.100`                                         |
| DHCP range end      | Network + `.200`                                         |
| DHCP lease time     | Department-based (see §4.5)                              |
| DHCP domain         | `{department}.local`                                     |
| DNS servers         | `[gateway_ip, "8.8.8.8", "1.1.1.1"]`                     |
| NTP servers         | `["0.pool.ntp.org", "1.pool.ntp.org", "2.pool.ntp.org"]` |
| Static reservations | Department-specific device list                          |

**VlanGenerator** state:

```go
type VlanGenerator struct {
    rng         *rand.Rand       // Seeded RNG
    usedVlanIDs map[uint16]bool  // Uniqueness tracking
    usedNets    map[string]bool  // Uniqueness tracking
}
```

**Generation methods**:

- `NewVlanGenerator(seed *int64) *VlanGenerator`
- `(g *VlanGenerator) GenerateSingle() (VlanConfig, error)`
- `(g *VlanGenerator) GenerateBatch(count int) ([]VlanConfig, error)`
- `(g *VlanGenerator) GenerateFromRanges(ranges []VlanRange) ([]VlanConfig, error)`

**WAN assignment strategies**:

- `Single`: All VLANs get WAN 1
- `Multi`: Round-robin across WANs 1-3
- `Balanced`: Random distribution across WANs 1-3

**Network generation algorithm**:

1. Pick random RFC 1918 class (weighted: 60% Class A, 25% Class B, 15% Class C)
2. Generate random /24 within that class
3. Check uniqueness against `usedNets`
4. Retry if collision (up to 100 attempts, then error)

### 4.2 Firewall Rule Generator

**Purpose**: Generate realistic firewall rules for each VLAN.

**FirewallComplexity** determines rules per VLAN:

| Level          | Rules | Description                                   |
| -------------- | ----- | --------------------------------------------- |
| `Basic`        | 3     | Essential traffic rules                       |
| `Intermediate` | 7     | Basic + network services                      |
| `Advanced`     | 15    | Intermediate + department-specific + security |

**Basic rules** (always generated):

1. Allow internal VLAN traffic (source=VLAN net, dest=VLAN net, proto=any, action=pass)
2. Allow DNS queries (dest=any, proto=UDP, port=53, action=pass)
3. Allow HTTP/HTTPS (dest=any, proto=TCP, ports=80,443, action=pass)

**Intermediate additions**: 4. Allow NTP (dest=any, proto=UDP, port=123, action=pass) 5. Allow ICMP diagnostics (proto=ICMP, action=pass) 6. Block common attack ports (dest=any, ports=23,445,3389, action=block) 7. Log denied traffic (source=any, dest=any, action=block, log=true)

**Advanced additions**: 8-15. Department-specific rules (e.g., IT gets SSH access, Dev gets high ports, etc.)

**Mapping to opnDossier `schema.Rule`**:

```go
// Faker generates these fields on schema.Rule:
rule := schema.Rule{
    Type:        "pass" | "block",      // maps from action
    Descr:       description,
    Interface:   InterfaceList{"optN"}, // VLAN interface
    IPProtocol:  "inet",
    Protocol:    "tcp" | "udp" | "icmp",
    Source:      schema.Source{Network: vlanNet} | schema.Source{Any: ptr("")},
    Destination: schema.Destination{...},
    Log:         schema.BoolFlag(log),
    Direction:   "in",                  // Faker only generates inbound
    Tracker:     uniqueTracker,         // Unique tracker ID
}
```

### 4.3 NAT Mapping Generator

**Purpose**: Generate realistic NAT rules tied to VLANs.

**NatRuleType** enum:

- `PortForward` — WAN to internal server (most common)
- `SourceNat` — Outbound traffic rewriting
- `DestinationNat` — Inbound traffic rewriting
- `OneToOneNat` — 1:1 IP mapping
- `OutboundNat` — Outbound NAT rules

**Generation logic**:

- Port forwards: Map external ports (80, 443, 8080, 22, 3389) to internal VLAN hosts
- Source NAT: Rewrite source for outbound traffic from VLAN subnets
- Each mapping gets a UUID, protocol (TCP/UDP/Both), and logging flag

**Mapping to opnDossier types**:

- `PortForward` → `schema.InboundRule` (with `InternalIP`, `InternalPort`, `ExternalPort`)
- `SourceNat` / `OutboundNat` → `schema.NATRule` (within `schema.Outbound`)
- `OneToOneNat` → `schema.NATRule` with `Target` field

### 4.4 VPN Configuration Generator

**Purpose**: Generate OpenVPN, WireGuard, or IPSec VPN configurations.

**VPN generation per type**:

| Type      | opnDossier Schema            | Key Fields                                                 |
| --------- | ---------------------------- | ---------------------------------------------------------- |
| OpenVPN   | `schema.OpenVPNServer`       | VPN ID, mode, protocol, port, tunnel network, certificates |
| WireGuard | `schema.WireGuardServerItem` | UUID, name, pubkey, privkey, port, tunnel address          |
| IPSec     | `schema.IPsec`               | General settings, charon config, pre-shared keys           |

**Generation rules**:

- OpenVPN: port range 1194-1294, UDP preferred, AES-256-GCM cipher
- WireGuard: port range 51820-51920, unique keypairs (fake), /24 tunnel networks
- IPSec: IKEv2, AES-256, SHA-256, DH group 14+
- Each VPN gets a unique tunnel subnet (non-overlapping with VLAN subnets)

### 4.5 Department System

**20 departments** used for VLAN descriptions and DHCP lease time differentiation:

```
Sales, IT, HR, Finance, Marketing, Operations, Engineering, Support,
Legal, Procurement, Security, Development, QA, Research, Training,
Management, Accounting, Customer Service, Logistics, Production
```

**DHCP lease time mapping**:

| Category      | Departments                                                           | Lease (seconds) |
| ------------- | --------------------------------------------------------------------- | --------------- |
| Corporate     | IT, Finance, Legal, Accounting, Management                            | 86400 (24h)     |
| Production    | Engineering, Development, QA, Research                                | 43200 (12h)     |
| Dynamic       | Sales, Marketing, Customer Service                                    | 28800 (8h)      |
| High-mobility | HR, Logistics, Training, Support, Operations, Procurement, Production | 14400 (4h)      |
| Security      | Security                                                              | 21600 (6h)      |

**Static reservation templates per department** (example):

- IT: printer, NAS, server (3 reservations)
- Engineering: build-server, CI-runner (2 reservations)
- Security: camera, access-controller (2 reservations)
- Others: 0-1 reservations

MAC addresses are generated in `AA:BB:CC:DD:EE:FF` format with deterministic octets.

---

## 5. XML Generation Pipeline

### 5.1 Data Flow

```
CLI args
  → VlanGenerator.GenerateBatch(count)
  → [for each vlan] FirewallGenerator.GenerateRules(vlan, complexity)
  → [optional] NatGenerator.GenerateMappings(vlans, natCount)
  → [optional] VpnGenerator.GenerateConfigs(vpnCount)
  → XmlTemplate.Load(baseConfigPath)
  → XmlTemplate.InjectVlans(vlans, firewallRules, natMappings, vpnConfigs)
  → XmlTemplate.Write(outputPath)
```

### 5.2 Template Injection

The base `config.xml` is a real OPNsense configuration. The faker injects generated data at specific XML paths:

| Injection Point | XML Path                 | Content                                  |
| --------------- | ------------------------ | ---------------------------------------- |
| VLANs           | `<opnsense><vlans>`      | `<vlan>` elements                        |
| Interfaces      | `<opnsense><interfaces>` | `<optN>` elements for each VLAN          |
| DHCP            | `<opnsense><dhcpd>`      | `<optN>` DHCP configs per VLAN           |
| Firewall rules  | `<opnsense><filter>`     | `<rule>` elements                        |
| NAT             | `<opnsense><nat>`        | `<outbound><rule>` and `<inbound><rule>` |
| OpenVPN         | `<opnsense><openvpn>`    | `<openvpn-server>` elements              |
| Gateways        | `<opnsense><gateways>`   | `<gateway_item>` elements                |

### 5.3 Template Placeholders

The Rust implementation supports these placeholders in XML templates:

- `{{VLAN_ID}}` — VLAN tag number
- `{{IP_NETWORK}}` — CIDR network
- `{{DESCRIPTION}}` — VLAN description
- `{{WAN_ASSIGNMENT}}` — WAN group number
- `{{FIREWALL_NR}}` — Firewall instance number
- `{{OPT_COUNTER}}` — Interface counter (opt6, opt7, ...)
- `{{GATEWAY_IP}}` — Gateway address
- `{{DHCP_START}}` — DHCP range start
- `{{DHCP_END}}` — DHCP range end

### 5.4 XML Generation Strategy

**For the Go rewrite**, use `encoding/xml` with opnDossier's schema types:

1. **Parse** base config into `schema.OpnSenseDocument` using `encoding/xml`
2. **Populate** generated data into the document's fields:
   - Append VLANs to `doc.VLANs.VLAN`
   - Add interfaces to `doc.Interfaces.Items["optN"]`
   - Add DHCP configs to `doc.Dhcpd.Items["optN"]`
   - Append rules to `doc.Filter.Rule`
   - Append NAT rules to `doc.Nat.Outbound.Rule` / `doc.Nat.Inbound`
3. **Marshal** the complete document back to XML with `xml.MarshalIndent`

This is simpler than the Rust approach (which uses event-based XML injection) because Go's `encoding/xml` + opnDossier's custom Marshal/Unmarshal handles the round-trip.

### 5.5 Output File Naming

Pattern: `firewall_{NR}_config.xml` where `{NR}` is the firewall number (1-based).

When generating multiple configs: `firewall_1_config.xml`, `firewall_2_config.xml`, etc.

---

## 6. Validation Rules

### 6.1 VLAN Validation

| Rule                  | Check                                             |
| --------------------- | ------------------------------------------------- |
| VLAN ID range         | 10 ≤ id ≤ 4094                                    |
| VLAN ID uniqueness    | No duplicates in generation run                   |
| Network uniqueness    | No duplicate subnets                              |
| RFC 1918 compliance   | Network must be in 10/8, 172.16/12, or 192.168/16 |
| WAN assignment        | 1 ≤ wan ≤ 3                                       |
| Description non-empty | Must have a description string                    |

### 6.2 Network Validation (RFC 1918)

```go
func IsRFC1918(network netip.Prefix) bool {
    addr := network.Addr()
    return rfc1918ClassA.Contains(addr) ||  // 10.0.0.0/8
           rfc1918ClassB.Contains(addr) ||  // 172.16.0.0/12
           rfc1918ClassC.Contains(addr)     // 192.168.0.0/16
}
```

### 6.3 Firewall Rule Validation

- Valid actions: `pass`, `block`, `reject`
- Valid directions: `in`, `out`
- Valid protocols: `tcp`, `udp`, `icmp`, `any`
- Port ranges: 1-65535 or named ranges
- Each rule needs a unique tracker ID

### 6.4 Cross-Object Consistency

- Every VLAN interface must have a corresponding DHCP config
- Firewall rules must reference existing interfaces
- NAT rules must reference valid VLANs/interfaces
- VPN tunnel subnets must not overlap with VLAN subnets
- Gateway interfaces must exist

---

## 7. CLI Specification

### 7.1 Command Structure

```
opnsense-config-faker generate [flags]
opnsense-config-faker validate [flags]
opnsense-config-faker completion [bash|zsh|fish|powershell]
```

### 7.2 `generate` Command Flags

| Flag                         | Type       | Default  | Description                                   |
| ---------------------------- | ---------- | -------- | --------------------------------------------- |
| `--format`                   | `csv\|xml` | required | Output format                                 |
| `--count`                    | `int`      | `10`     | Number of VLANs (1-10000)                     |
| `--output`                   | `string`   | stdout   | Output file path                              |
| `--base-config`              | `string`   | —        | Base OPNsense XML template (required for XML) |
| `--csv-file`                 | `string`   | —        | Read VLANs from existing CSV                  |
| `--firewall-nr`              | `int`      | `1`      | Firewall instance number (1-999)              |
| `--opt-counter`              | `int`      | `6`      | Starting interface counter                    |
| `--force`                    | `bool`     | `false`  | Overwrite existing files                      |
| `--seed`                     | `int64`    | random   | RNG seed for reproducibility                  |
| `--interactive`              | `bool`     | `false`  | Prompt for missing arguments                  |
| `--include-firewall-rules`   | `bool`     | `false`  | Generate firewall rules                       |
| `--firewall-rule-complexity` | `string`   | `basic`  | `basic\|intermediate\|advanced`               |
| `--vlan-range`               | `string`   | —        | VLAN range spec, e.g. `100-150,200-250`       |
| `--vpn-count`                | `int`      | `0`      | Number of VPN configs                         |
| `--nat-mappings`             | `int`      | `0`      | Number of NAT rules                           |
| `--wan-assignments`          | `string`   | `single` | `single\|multi\|balanced`                     |

### 7.3 `validate` Command Flags

| Flag           | Type     | Default     | Description         |
| -------------- | -------- | ----------- | ------------------- |
| `--input`      | `string` | required    | File to validate    |
| `--format`     | `string` | auto-detect | `csv\|xml`          |
| `--max-errors` | `int`    | `10`        | Stop after N errors |

### 7.4 Global Flags

| Flag         | Type     | Default | Description                   |
| ------------ | -------- | ------- | ----------------------------- |
| `--quiet`    | `bool`   | `false` | Suppress non-essential output |
| `--no-color` | `bool`   | `false` | Disable colored output        |
| `--output`   | `string` | —       | Global output path            |

### 7.5 Environment Variable Support

| Variable                 | Effect                           |
| ------------------------ | -------------------------------- |
| `TERM=dumb`              | Disable progress bars and colors |
| `NO_COLOR=1`             | Disable colors                   |
| `CARGO_TERM_COLOR=never` | Disable colors (legacy compat)   |

---

## 8. CSV Format Specification

### 8.1 VLAN CSV

Header row uses German field names (legacy):

```csv
VLAN,IP Range,Beschreibung,WAN
42,10.42.0.0/24,IT VLAN 42,1
100,10.100.0.0/24,Sales VLAN 100,2
```

| Column         | Type     | Description                            |
| -------------- | -------- | -------------------------------------- |
| `VLAN`         | `u16`    | VLAN ID (10-4094)                      |
| `IP Range`     | `string` | CIDR network                           |
| `Beschreibung` | `string` | Description (German for "description") |
| `WAN`          | `u8`     | WAN assignment (1-3)                   |

### 8.2 Firewall Rules CSV

```csv
rule_id,source,destination,protocol,ports,action,direction,description,log,vlan_id,priority,interface
```

| Column        | Type     | Description                                  |
| ------------- | -------- | -------------------------------------------- |
| `rule_id`     | `string` | Unique rule identifier                       |
| `source`      | `string` | Source network/IP                            |
| `destination` | `string` | Destination network/IP                       |
| `protocol`    | `string` | tcp/udp/icmp/any                             |
| `ports`       | `string` | Port or range (e.g., "80,443", "1024:65535") |
| `action`      | `string` | pass/block/reject                            |
| `direction`   | `string` | in/out                                       |
| `description` | `string` | Human-readable description                   |
| `log`         | `bool`   | Enable logging                               |
| `vlan_id`     | `u16?`   | Associated VLAN (optional)                   |
| `priority`    | `u16`    | Rule order priority                          |
| `interface`   | `string` | Interface name (e.g., "vlan42")              |

---

## 9. Error Handling

### 9.1 Error Types to Implement

```go
package errors

import "fmt"

type ConfigError struct {
    Kind    ErrorKind
    Message string
    Cause   error
}

type ErrorKind int

const (
    ErrIO ErrorKind = iota
    ErrCSV
    ErrXML
    ErrNetwork
    ErrVlanGeneration
    ErrValidation
    ErrXmlTemplate
    ErrXmlInjectionPointNotFound
    ErrXmlSchemaValidation
    ErrXmlMemoryLimitExceeded
    ErrConfigNotFound
    ErrInvalidParameter
    ErrResourceExhausted
)
```

### 9.2 VLAN-Specific Errors

```go
type VlanError struct {
    Kind    VlanErrorKind
    Message string
}

type VlanErrorKind int

const (
    ErrInvalidVlanID VlanErrorKind = iota    // ID outside 10-4094
    ErrNonRFC1918Network                      // Network not in RFC 1918
    ErrNetworkParsing                         // Failed to parse network
    ErrVlanIDExhausted                        // All 4085 IDs used
    ErrNetworkExhausted                       // All unique networks used
    ErrInvalidWanAssignment                   // WAN outside 1-3
    ErrInvalidDepartment                      // Unknown department
    ErrValidationFailed                       // General validation failure
)
```

---

## 10. Performance Targets

| Metric                     | Target              | Notes                         |
| -------------------------- | ------------------- | ----------------------------- |
| Throughput                 | ≥150 configs/second | 3x Python baseline            |
| Memory per config          | ≤25 KB              | Average VLAN block ~256 bytes |
| Memory limit               | 32 MB configurable  | For XML processing            |
| Small datasets (\<100)     | \<50ms              |                               |
| Medium datasets (100-1000) | \<500ms             |                               |
| Large datasets (>1000)     | \<2s                |                               |

### 10.1 Go-Specific Optimizations

- Use `sync.Pool` for temporary buffers
- Use `netip.Prefix` instead of string parsing for network operations
- Pre-allocate slices with `make([]T, 0, expectedCap)`
- Use `strings.Builder` for XML template assembly
- Consider `errgroup` for parallel generation of independent config components

---

## 11. Testing Strategy

### 11.1 Test Types to Port

| Rust Test Type               | Go Equivalent                                     | Location                                   |
| ---------------------------- | ------------------------------------------------- | ------------------------------------------ |
| Unit tests (co-located)      | `*_test.go` in same package                       | `internal/generator/vlan_test.go` etc.     |
| Integration tests (`tests/`) | `*_test.go` in `cmd/` or separate `_test` package | `cmd/generate_test.go`                     |
| Property tests (proptest)    | `testing/quick` or `rapid`                        | `internal/generator/vlan_property_test.go` |
| Snapshot tests (insta)       | `cupaloy` or `goldie`                             | `internal/xmlgen/snapshot_test.go`         |
| Benchmarks (criterion)       | `testing.B`                                       | `internal/generator/vlan_bench_test.go`    |

### 11.2 Key Test Cases to Preserve

**VLAN Generation**:

- Seeded generation produces deterministic output
- All VLAN IDs unique across batch
- All networks unique across batch
- VLAN IDs within 10-4094
- Networks are RFC 1918 compliant
- Max generation count (4085) succeeds
- Count >4085 returns ResourceExhausted error
- VLAN range parsing ("100-150,200-250")
- WAN assignment strategies produce correct distribution

**Firewall Rules**:

- Basic complexity → exactly 3 rules per VLAN
- Intermediate complexity → exactly 7 rules per VLAN
- Advanced complexity → exactly 15 rules per VLAN
- All rules reference valid interfaces
- Rule actions are valid (pass/block/reject)

**XML Generation**:

- Round-trip: parse → inject → marshal preserves base config
- Generated XML validates against opnsense-config.xsd
- DHCP ranges are within VLAN subnets
- Interface names follow optN pattern
- Special characters are XML-escaped
- German umlauts are handled

**CSV I/O**:

- Round-trip: write → read produces identical data
- German header names ("Beschreibung", not "Description")
- Invalid VLAN IDs rejected on read
- Invalid networks rejected on read

---

## 12. Migration Checklist

### Phase 1: Foundation

- [ ] Set up Go module with opnDossier schema dependency
- [ ] Implement `internal/netutil/rfc1918.go` (RFC 1918 utilities)
- [ ] Implement `internal/generator/departments.go` (20 departments + lease times)
- [ ] Implement `internal/errors/` (error types)

### Phase 2: Core Generators

- [ ] Implement `internal/generator/vlan.go` (VLAN generation)
- [ ] Implement `internal/generator/firewall.go` (firewall rule generation)
- [ ] Implement `internal/generator/nat.go` (NAT mapping generation)
- [ ] Implement `internal/generator/vpn.go` (VPN config generation)
- [ ] Implement `internal/validate/` (validation engine)

### Phase 3: I/O Layer

- [ ] Implement `internal/csvio/` (CSV read/write with German headers)
- [ ] Implement `internal/xmlgen/` (XML template + injection)
- [ ] Verify round-trip with opnDossier schema types

### Phase 4: CLI

- [ ] Implement `cmd/generate.go` (Cobra command)
- [ ] Implement `cmd/validate.go` (Cobra command)
- [ ] Implement `cmd/completion.go` (shell completions)
- [ ] Implement `cmd/root.go` (global flags, env var support)

### Phase 5: Quality

- [ ] Port all unit tests
- [ ] Port property-based tests
- [ ] Port snapshot tests
- [ ] Add benchmarks
- [ ] Verify ≥80% test coverage
- [ ] Verify zero `golangci-lint` warnings
- [ ] Verify performance targets met

### Phase 6: Integration

- [ ] Verify shared schema types work for both faker and opnDossier
- [ ] Cross-platform testing (macOS, Linux, Windows)
- [ ] CI/CD pipeline (GitHub Actions)
- [ ] GoReleaser configuration (match opnDossier pattern)

---

## 13. Files to Reference During Rewrite

### Critical Rust Source Files

| File                           | Lines | What to Extract                                                      |
| ------------------------------ | ----- | -------------------------------------------------------------------- |
| `src/generator/vlan.rs`        | 1,668 | VLAN generation algorithm, DHCP derivation logic, network generation |
| `src/generator/firewall.rs`    | ~400  | Rule templates per complexity level, department-specific rules       |
| `src/generator/nat.rs`         | ~300  | NAT rule templates, port forward patterns                            |
| `src/generator/vpn.rs`         | ~300  | VPN config generation per type                                       |
| `src/generator/departments.rs` | ~100  | Department list, lease time mapping                                  |
| `src/validate/mod.rs`          | 225   | Validation rules and uniqueness tracking                             |
| `src/utils/rfc1918.rs`         | 261   | RFC 1918 validation and generation                                   |
| `src/io/csv.rs`                | 788   | CSV format, German headers, validation sets                          |
| `src/xml/template.rs`          | ~200  | Template placeholder system                                          |
| `src/xml/streaming.rs`         | ~300  | Streaming XML generation approach                                    |
| `src/cli/mod.rs`               | 463   | CLI argument structure and defaults                                  |
| `src/cli/commands/generate.rs` | 683   | Generation workflow orchestration                                    |

### opnDossier Reference Files

| File                            | What It Provides                               |
| ------------------------------- | ---------------------------------------------- |
| `internal/schema/opnsense.go`   | Root document structure                        |
| `internal/schema/interfaces.go` | Interface + VLAN types with custom XML marshal |
| `internal/schema/security.go`   | Firewall Rule, NAT, Source, Destination types  |
| `internal/schema/dhcp.go`       | DHCP types with map-based interface support    |
| `internal/schema/vpn.go`        | OpenVPN, WireGuard, IPSec types                |
| `internal/schema/network.go`    | Gateway, StaticRoute types                     |
| `internal/schema/common.go`     | BoolFlag, helper types                         |
| `internal/cfgparser/xml.go`     | XML parsing with encoding support              |
| `go.mod`                        | Dependency versions to align with              |
