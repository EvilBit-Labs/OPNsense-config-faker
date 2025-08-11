# Follow-up Issue: Extend XML Template Coverage as XML Module Grows

**Issue Type**: Enhancement\
**Priority**: Medium\
**Labels**: `enhancement`, `xml`, `templates`, `testing`

## Summary

As the XML module continues to grow with new OPNsense configuration elements, we need to ensure comprehensive template coverage for all supported XML generation features.

## Background

With the current testing framework in place, we have established a solid foundation for XML generation testing. However, as we add support for more OPNsense configuration elements, we need to:

1. Expand template coverage for new XML elements
2. Ensure all XML generation paths are properly tested
3. Maintain comprehensive snapshot testing for template variations

## Proposed Work

### Template Coverage Expansion

- [ ] **Interface Templates**: Extend coverage for additional interface types (LAGG, Bridge, VXLAN)
- [ ] **Firewall Rule Templates**: Add support for more complex rule scenarios (floating rules, NAT reflection)
- [ ] **DHCP Templates**: Expand DHCP server configuration options (static mappings, options)
- [ ] **VPN Templates**: Add OpenVPN and IPSec configuration templates
- [ ] **Traffic Shaping Templates**: Include limiter and queue configurations
- [ ] **System Templates**: Cover cron jobs, gateways, and routing configurations

### Testing Enhancements

- [ ] **Template Validation**: Ensure all templates produce valid OPNsense XML
- [ ] **Cross-reference Testing**: Verify that generated elements reference each other correctly
- [ ] **Schema Compliance**: Add automated XSD validation for all generated XML
- [ ] **Edge Case Coverage**: Test boundary conditions and error scenarios

### Documentation

- [ ] **Template Documentation**: Document all available template options and parameters
- [ ] **Coverage Reports**: Generate reports showing which OPNsense elements are covered
- [ ] **Migration Guide**: Document how to add new template types

## Acceptance Criteria

- [ ] All new XML generation features have corresponding templates
- [ ] Template test coverage remains above 95%
- [ ] Generated XML validates against OPNsense XSD schema
- [ ] Documentation is updated for all new templates
- [ ] Snapshot tests cover all template variations

## Implementation Notes

### Template Organization

```text
templates/
├── interfaces/
│   ├── basic.xml.j2
│   ├── lagg.xml.j2
│   └── bridge.xml.j2
├── firewall/
│   ├── rules.xml.j2
│   └── floating_rules.xml.j2
├── dhcp/
│   ├── server.xml.j2
│   └── static_mappings.xml.j2
└── vpn/
    ├── openvpn.xml.j2
    └── ipsec.xml.j2
```

### Testing Strategy

- Use property-based testing for template parameter validation
- Implement comprehensive snapshot testing for all template combinations
- Add integration tests that validate generated XML against real OPNsense instances

### Dependencies

- XSD schema files for validation
- Jinja2 template engine enhancements
- Additional test fixtures and mock data

## Timeline

- **Phase 1** (2 weeks): Interface and firewall rule template expansion
- **Phase 2** (2 weeks): DHCP and VPN template development
- **Phase 3** (1 week): Documentation and testing refinements

## Related Issues

- Link to any existing issues about XML generation
- Reference the original testing framework implementation
- Connect to broader template system architecture discussions
