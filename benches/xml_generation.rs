use criterion::{criterion_group, criterion_main, Criterion};
use opnsense_config_faker::generator::vlan::generate_vlan_configurations;
use opnsense_config_faker::xml::template::XmlTemplate;
use std::hint::black_box;

fn bench_xml_generation(c: &mut Criterion) {
    let base_xml = r#"<?xml version="1.0"?>
<opnsense>
    <vlan id="{{VLAN_ID}}">{{DESCRIPTION}}</vlan>
    <network>{{IP_NETWORK}}</network>
    <gateway>{{GATEWAY_IP}}</gateway>
</opnsense>"#;

    c.bench_function("xml_template_apply_single", |b| {
        let configs = generate_vlan_configurations(1, Some(42), None).unwrap();
        let config = &configs[0];

        b.iter(|| {
            let mut template = XmlTemplate::new(base_xml.to_string()).unwrap();
            let result = template
                .apply_configuration(black_box(config), 1, 6)
                .unwrap();
            black_box(result)
        })
    });

    c.bench_function("xml_template_apply_100", |b| {
        let configs = generate_vlan_configurations(100, Some(42), None).unwrap();

        b.iter(|| {
            let mut results = Vec::new();
            for config in &configs {
                let mut template = XmlTemplate::new(base_xml.to_string()).unwrap();
                let result = template
                    .apply_configuration(black_box(config), 1, 6)
                    .unwrap();
                results.push(result);
            }
            black_box(results)
        })
    });
}

criterion_group!(benches, bench_xml_generation);
criterion_main!(benches);
