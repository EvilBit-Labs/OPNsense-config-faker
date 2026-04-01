# Configuration Schema

OPNsense configuration schema and validation rules for generated configurations.

## XML Schema

### Root Element

```xml
<opnsense>
  
</opnsense>
```

### VLAN Configuration

```xml
<vlans>
  <vlan>
    <vlanif>vlan100</vlanif>
    <tag>100</tag>
    <descr>IT Department VLAN</descr>
    <if>em0</if>
  </vlan>
</vlans>
```

**Elements**:

- `vlanif`: VLAN interface name (e.g., "vlan100")
- `tag`: VLAN ID (1-4094)
- `descr`: VLAN description
- `if`: Parent interface name

### Interface Configuration

```xml
<interfaces>
  <vlan100>
    <enable>1</enable>
    <if>vlan100</if>
    <descr>IT Department VLAN</descr>
    <ipaddr>192.168.100.1</ipaddr>
    <subnet>24</subnet>
  </vlan100>
</interfaces>
```

**Elements**:

- `enable`: Interface enabled (1) or disabled (0)
- `if`: Interface name
- `descr`: Interface description
- `ipaddr`: IP address
- `subnet`: Subnet mask in CIDR notation

### Firewall Rules

```xml
<filter>
  <rule>
    <id>1</id>
    <type>pass</type>
    <interface>vlan100</interface>
    <ipprotocol>inet</ipprotocol>
    <protocol>tcp</protocol>
    <source>
      <address>192.168.100.0/24</address>
    </source>
    <destination>
      <address>any</address>
      <port>80</port>
    </destination>
    <descr>Allow HTTP traffic</descr>
  </rule>
</filter>
```

**Elements**:

- `id`: Rule identifier
- `type`: Rule action (pass, block, reject)
- `interface`: Source interface
- `ipprotocol`: IP protocol (inet, inet6)
- `protocol`: Transport protocol (tcp, udp, icmp, etc.)
- `source`: Source address and port
- `destination`: Destination address and port
- `descr`: Rule description

### DHCP Configuration

```xml
<dhcpd>
  <vlan100>
    <enable>1</enable>
    <range>
      <from>192.168.100.100</from>
      <to>192.168.100.200</to>
    </range>
    <defaultleasetime>7200</defaultleasetime>
    <maxleasetime>86400</maxleasetime>
    <netmask>255.255.255.0</netmask>
    <gateway>192.168.100.1</gateway>
    <domain>example.com</domain>
    <domainsearchlist>example.com</domainsearchlist>
    <dnsserver>8.8.8.8</dnsserver>
    <dnsserver>8.8.4.4</dnsserver>
  </vlan100>
</dhcpd>
```

**Elements**:

- `enable`: DHCP enabled (1) or disabled (0)
- `range`: IP address range
- `defaultleasetime`: Default lease time in seconds
- `maxleasetime`: Maximum lease time in seconds
- `netmask`: Subnet mask
- `gateway`: Gateway IP address
- `domain`: Domain name
- `domainsearchlist`: Domain search list
- `dnsserver`: DNS server addresses

### NAT Rules

```xml
<nat>
  <rule>
    <id>1</id>
    <type>pass</type>
    <interface>wan</interface>
    <ipprotocol>inet</ipprotocol>
    <protocol>tcp</protocol>
    <source>
      <address>192.168.100.0/24</address>
    </source>
    <destination>
      <address>any</address>
      <port>80</port>
    </destination>
    <target>192.168.1.100</target>
    <targetport>8080</targetport>
    <descr>Port forwarding rule</descr>
  </rule>
</nat>
```

**Elements**:

- `id`: Rule identifier
- `type`: Rule action (pass, block, reject)
- `interface`: Source interface
- `ipprotocol`: IP protocol (inet, inet6)
- `protocol`: Transport protocol
- `source`: Source address and port
- `destination`: Destination address and port
- `target`: Target IP address
- `targetport`: Target port
- `descr`: Rule description

## Validation Rules

### VLAN Validation

- **VLAN ID**: Must be between 1 and 4094
- **Interface Name**: Must follow format "vlan{id}"
- **Parent Interface**: Must be a valid physical interface
- **Network Range**: Must be a valid IP network

### Interface Validation

- **IP Address**: Must be a valid IPv4 address
- **Subnet Mask**: Must be a valid CIDR notation (8-30)
- **Gateway**: Must be within the network range
- **DNS Servers**: Must be valid IP addresses

### Firewall Rule Validation

- **Rule ID**: Must be unique within the configuration
- **Action**: Must be "pass", "block", or "reject"
- **Protocol**: Must be a valid transport protocol
- **Addresses**: Must be valid IP addresses or networks
- **Ports**: Must be valid port numbers (1-65535)

### DHCP Validation

- **IP Range**: Start and end addresses must be valid
- **Lease Times**: Must be positive integers
- **Network Mask**: Must match the interface subnet
- **Gateway**: Must be within the network range
- **DNS Servers**: Must be valid IP addresses

### NAT Rule Validation

- **Rule ID**: Must be unique within the configuration
- **Target**: Must be a valid IP address
- **Port Mapping**: Source and target ports must be valid
- **Interface**: Must be a valid interface name

## Network Constraints

### IP Address Ranges

- **Private Networks**: Use RFC 1918 private address spaces
  - 10.0.0.0/8 (Class A)
  - 172.16.0.0/12 (Class B)
  - 192.168.0.0/16 (Class C)
- **Subnet Sizes**: Minimum /30, maximum /8
- **No Overlap**: Network ranges must not overlap

### VLAN Constraints

- **VLAN ID Range**: 1-4094 (IEEE 802.1Q standard)
- **Unique IDs**: No duplicate VLAN IDs within configuration
- **Interface Naming**: Must follow "vlan{id}" format
- **Parent Interface**: Must be a valid physical interface

### Port Constraints

- **Port Range**: 1-65535
- **Well-Known Ports**: 1-1023
- **Registered Ports**: 1024-49151
- **Dynamic Ports**: 49152-65535

## Schema Validation

### XML Schema Definition

```xml
<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema">
  <xs:element name="opnsense">
    <xs:complexType>
      <xs:sequence>
        <xs:element name="vlans" minOccurs="0">
          <xs:complexType>
            <xs:sequence>
              <xs:element name="vlan" maxOccurs="unbounded">
                <xs:complexType>
                  <xs:sequence>
                    <xs:element name="vlanif" type="xs:string" />
                    <xs:element name="tag" type="xs:unsignedShort" />
                    <xs:element name="descr" type="xs:string" />
                    <xs:element name="if" type="xs:string" />
                  </xs:sequence>
                </xs:complexType>
              </xs:element>
            </xs:sequence>
          </xs:complexType>
        </xs:element>
      </xs:sequence>
    </xs:complexType>
  </xs:element>
</xs:schema>
```

### Validation Process

1. **XML Well-Formedness**: Check XML syntax
2. **Schema Validation**: Validate against XSD schema
3. **Business Logic**: Check configuration constraints
4. **Network Validation**: Verify network ranges and addresses
5. **Consistency Check**: Ensure configuration consistency

## Configuration Examples

### Minimal VLAN Configuration

```xml
<opnsense>
  <vlans>
    <vlan>
      <vlanif>vlan100</vlanif>
      <tag>100</tag>
      <descr>IT Department VLAN</descr>
      <if>em0</if>
    </vlan>
  </vlans>
</opnsense>
```

### Complete Configuration

```xml
<opnsense>
  <vlans>
    <vlan>
      <vlanif>vlan100</vlanif>
      <tag>100</tag>
      <descr>IT Department VLAN</descr>
      <if>em0</if>
    </vlan>
  </vlans>
  <interfaces>
    <vlan100>
      <enable>1</enable>
      <if>vlan100</if>
      <descr>IT Department VLAN</descr>
      <ipaddr>192.168.100.1</ipaddr>
      <subnet>24</subnet>
    </vlan100>
  </interfaces>
  <filter>
    <rule>
      <id>1</id>
      <type>pass</type>
      <interface>vlan100</interface>
      <ipprotocol>inet</ipprotocol>
      <protocol>tcp</protocol>
      <source>
        <address>192.168.100.0/24</address>
      </source>
      <destination>
        <address>any</address>
        <port>80</port>
      </destination>
      <descr>Allow HTTP traffic</descr>
    </rule>
  </filter>
  <dhcpd>
    <vlan100>
      <enable>1</enable>
      <range>
        <from>192.168.100.100</from>
        <to>192.168.100.200</to>
      </range>
      <defaultleasetime>7200</defaultleasetime>
      <maxleasetime>86400</maxleasetime>
      <netmask>255.255.255.0</netmask>
      <gateway>192.168.100.1</gateway>
      <domain>example.com</domain>
      <dnsserver>8.8.8.8</dnsserver>
    </vlan100>
  </dhcpd>
</opnsense>
```

## Error Handling

### Validation Errors

- **Invalid VLAN ID**: VLAN ID outside valid range
- **Network Overlap**: Overlapping network ranges
- **Invalid IP Address**: Malformed IP addresses
- **Port Range Error**: Invalid port numbers
- **Schema Violation**: XML structure violations

### Error Recovery

1. **Identify Error**: Determine the specific validation failure
2. **Fix Configuration**: Correct the invalid values
3. **Re-validate**: Run validation again
4. **Generate Report**: Create detailed error report

## Best Practices

### Configuration Design

1. **Use Descriptive Names**: Clear, meaningful names for all elements
2. **Logical Grouping**: Group related configurations together
3. **Consistent Naming**: Follow consistent naming conventions
4. **Documentation**: Include descriptions for all elements

### Network Design

1. **Non-Overlapping Ranges**: Ensure network ranges don't conflict
2. **Logical Subnetting**: Use logical subnet allocation
3. **Reserved Addresses**: Reserve addresses for infrastructure
4. **Future Growth**: Plan for network expansion

### Security Considerations

1. **Least Privilege**: Use minimal required permissions
2. **Explicit Rules**: Avoid overly permissive rules
3. **Logging**: Enable logging for security events
4. **Regular Review**: Periodically review configurations

This configuration schema provides the foundation for generating valid, consistent OPNsense configurations that can be imported and used in production environments.
