#[path = "_common/mod.rs"]
mod bench_common;

use bench_common::{ci_or_local, criterion_for_env};
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

    // Use CI-appropriate dataset sizes for XML template operations
    let batch_sizes = ci_or_local(&[50], &[100]);
    for &size in &batch_sizes {
        c.bench_function(&format!("xml_template_apply_{}", size), |b| {
            let configs = generate_vlan_configurations(size, Some(42), None).unwrap();

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
}

criterion_group! {
    name = benches;
    config = criterion_for_env();
    targets = bench_xml_generation
}
criterion_main!(benches);
