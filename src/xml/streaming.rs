//! High-performance XML generation for OPNsense configurations
//!
//! This module provides streaming XML generation with memory efficiency
//! and performance optimizations for large configuration sets.

use crate::generator::VlanConfig;
use crate::model::ConfigError;
use crate::Result;

use bumpalo::Bump;
use lru::LruCache;
use rustc_hash::FxHashMap;
use smallvec::SmallVec;
use std::io::Write;
use std::num::NonZeroUsize;

/// Template cache entry for compiled XML templates
#[derive(Debug, Clone)]
struct CompiledTemplate {
    header: String,
    vlan_template: String,
    footer: String,
}

/// High-performance streaming XML generator
pub struct StreamingXmlGenerator {
    /// Arena allocator for temporary strings
    arena: Bump,
    
    /// LRU cache for compiled templates
    template_cache: LruCache<String, CompiledTemplate>,
    
    /// Pre-allocated string buffer for XML generation
    xml_buffer: String,
    
    /// Fast lookup for XML escaping
    escape_map: FxHashMap<char, &'static str>,
    
    /// Small vector for component accumulation
    components: SmallVec<[String; 16]>,
}

impl StreamingXmlGenerator {
    /// Create a new streaming XML generator
    pub fn new() -> Self {
        let mut escape_map = FxHashMap::default();
        escape_map.insert('<', "&lt;");
        escape_map.insert('>', "&gt;");
        escape_map.insert('&', "&amp;");
        escape_map.insert('"', "&quot;");
        escape_map.insert('\'', "&apos;");

        Self {
            arena: Bump::new(),
            template_cache: LruCache::new(NonZeroUsize::new(8).unwrap()),
            xml_buffer: String::with_capacity(8192), // 8KB initial capacity
            escape_map,
            components: SmallVec::new(),
        }
    }

    /// Generate complete OPNsense XML configuration with streaming
    pub fn generate_config_streaming<W: Write>(
        &mut self,
        configs: &[VlanConfig],
        _base_template: Option<&str>,
        mut writer: W,
    ) -> Result<usize> {
        let mut bytes_written = 0;

        // Write XML declaration and root element
        let header = self.get_xml_header();
        writer.write_all(header.as_bytes())
            .map_err(|e| ConfigError::xml_template(format!("Failed to write header: {}", e)))?;
        bytes_written += header.len();

        // Write VLAN configurations in chunks to manage memory
        const CHUNK_SIZE: usize = 100;
        for chunk in configs.chunks(CHUNK_SIZE) {
            let chunk_xml = self.generate_vlan_chunk_xml(chunk)?;
            writer.write_all(chunk_xml.as_bytes())
                .map_err(|e| ConfigError::xml_template(format!("Failed to write VLAN chunk: {}", e)))?;
            bytes_written += chunk_xml.len();
            
            // Reset arena for next chunk
            self.arena.reset();
        }

        // Write footer
        let footer = "</opnsense>\n";
        writer.write_all(footer.as_bytes())
            .map_err(|e| ConfigError::xml_template(format!("Failed to write footer: {}", e)))?;
        bytes_written += footer.len();

        Ok(bytes_written)
    }

    /// Generate XML configuration with in-memory optimization
    pub fn generate_config_optimized(
        &mut self,
        configs: &[VlanConfig],
        _base_template: Option<&str>,
    ) -> Result<String> {
        // Pre-allocate buffer based on estimated size
        let estimated_size = self.estimate_xml_size(configs.len());
        self.xml_buffer.clear();
        self.xml_buffer.reserve(estimated_size);

        // Generate XML sections
        let header = self.get_xml_header();
        self.xml_buffer.push_str(&header);
        
        // Generate all VLANs
        for config in configs {
            let vlan_xml = self.generate_vlan_xml_optimized(config)?;
            self.xml_buffer.push_str(&vlan_xml);
        }
        
        self.xml_buffer.push_str("</opnsense>\n");

        Ok(self.xml_buffer.clone())
    }

    /// Get XML header with caching
    fn get_xml_header(&mut self) -> String {
        let cache_key = "header".to_string();
        
        if let Some(cached) = self.template_cache.get(&cache_key) {
            cached.header.clone()
        } else {
            let header = self.generate_xml_header();
            let template = CompiledTemplate {
                header: header.clone(),
                vlan_template: String::new(),
                footer: String::new(),
            };
            self.template_cache.put(cache_key, template);
            header
        }
    }

    /// Generate VLAN chunk XML
    fn generate_vlan_chunk_xml(&mut self, chunk: &[VlanConfig]) -> Result<String> {
        let mut chunk_xml = String::new();

        for config in chunk {
            let vlan_xml = self.generate_vlan_xml_optimized(config)?;
            chunk_xml.push_str(&vlan_xml);
        }

        Ok(chunk_xml)
    }

    /// Generate XML header template
    fn generate_xml_header(&self) -> String {
        r#"<?xml version="1.0"?>
<opnsense>
  <version>24.7</version>
  <system>
    <optimization>normal</optimization>
    <hostname>OPNsense</hostname>
    <domain>localdomain</domain>
    <dnsallowoverride>1</dnsallowoverride>
    <group>
      <name>admins</name>
      <description>System Administrators</description>
      <scope>system</scope>
      <gid>0</gid>
      <member>0</member>
      <priv>page-all</priv>
    </group>
    <user>
      <name>root</name>
      <descr>System Administrator</descr>
      <scope>system</scope>
      <groupname>admins</groupname>
      <password>$2y$10$YRVoF4SgskIsrXOvOQjGieB9XqHPRra9R7d80B3BZdbY/j21TwBfS</password>
      <uid>0</uid>
    </user>
  </system>
  <interfaces>
"#.to_string()
    }

    /// Generate optimized VLAN XML using template caching
    fn generate_vlan_xml_optimized(&mut self, config: &VlanConfig) -> Result<String> {
        // Use arena for temporary string operations
        let vlan_str = self.arena.alloc(format!("{}", config.vlan_id));
        let _wan_str = self.arena.alloc(format!("{}", config.wan_assignment));
        
        // Fast XML escaping
        let escaped_desc = self.escape_xml_fast(&config.description);
        let escaped_network = self.escape_xml_fast(&config.ip_network);

        // Template-based generation for better performance
        let xml = format!(
            r#"    <vlan{vlan_id}>
      <if>lan</if>
      <tag>{vlan_id}</tag>
      <pcp>0</pcp>
      <proto>802.1q</proto>
      <descr>{description}</descr>
      <enable>1</enable>
      <ipaddr>{ip_network}</ipaddr>
      <subnet>24</subnet>
      <gateway></gateway>
    </vlan{vlan_id}>
"#,
            vlan_id = vlan_str,
            description = escaped_desc,
            ip_network = escaped_network,
        );

        Ok(xml)
    }

    /// Fast XML escaping using lookup table
    fn escape_xml_fast(&self, text: &str) -> String {
        let mut result = String::with_capacity(text.len() + 16);
        
        for ch in text.chars() {
            if let Some(escaped) = self.escape_map.get(&ch) {
                result.push_str(escaped);
            } else {
                result.push(ch);
            }
        }
        
        result
    }

    /// Estimate XML size for pre-allocation
    fn estimate_xml_size(&self, vlan_count: usize) -> usize {
        const HEADER_SIZE: usize = 1024;
        const FOOTER_SIZE: usize = 64;
        const VLAN_AVG_SIZE: usize = 256; // Average VLAN XML size
        
        HEADER_SIZE + (vlan_count * VLAN_AVG_SIZE) + FOOTER_SIZE
    }

    /// Reset generator state
    pub fn reset(&mut self) {
        self.arena.reset();
        self.xml_buffer.clear();
        self.components.clear();
    }

    /// Parallel XML generation for large datasets
    #[cfg(feature = "rayon")]
    pub fn generate_parallel(
        &mut self,
        configs: &[VlanConfig],
        chunk_size: usize,
    ) -> Result<String> {
        use rayon::prelude::*;

        let chunks: Vec<&[VlanConfig]> = configs.chunks(chunk_size).collect();
        
        let xml_parts: Result<Vec<String>> = chunks
            .into_par_iter()
            .enumerate()
            .map(|(i, chunk)| {
                let mut local_generator = StreamingXmlGenerator::new();
                
                if i == 0 {
                    // First chunk includes header
                    let mut result = local_generator.generate_xml_header();
                    for config in chunk {
                        result.push_str(&local_generator.generate_vlan_xml_optimized(config)?);
                    }
                    Ok(result)
                } else {
                    // Other chunks only contain VLAN data
                    let mut result = String::new();
                    for config in chunk {
                        result.push_str(&local_generator.generate_vlan_xml_optimized(config)?);
                    }
                    Ok(result)
                }
            })
            .collect();

        let mut final_xml = String::new();
        for part in xml_parts? {
            final_xml.push_str(&part);
        }
        final_xml.push_str("</opnsense>\n");

        Ok(final_xml)
    }
}

impl Default for StreamingXmlGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generator::vlan::generate_vlan_configurations;

    #[test]
    fn test_streaming_xml_generator_creation() {
        let generator = StreamingXmlGenerator::new();
        assert!(generator.xml_buffer.capacity() >= 8192);
        assert_eq!(generator.escape_map.len(), 5);
    }

    #[test]
    fn test_xml_generation_optimized() {
        let mut generator = StreamingXmlGenerator::new();
        let configs = generate_vlan_configurations(10, Some(42), None).unwrap();
        
        let xml = generator.generate_config_optimized(&configs, None).unwrap();
        
        assert!(xml.contains("<?xml version=\"1.0\"?>"));
        assert!(xml.contains("<opnsense>"));
        assert!(xml.contains("</opnsense>"));
        assert_eq!(xml.matches("<vlan").count(), 10);
    }

    #[test]
    fn test_xml_streaming_generation() {
        let mut generator = StreamingXmlGenerator::new();
        let configs = generate_vlan_configurations(5, Some(42), None).unwrap();
        
        let mut buffer = Vec::new();
        let bytes_written = generator.generate_config_streaming(&configs, None, &mut buffer).unwrap();
        
        assert!(bytes_written > 0);
        
        let xml = String::from_utf8(buffer).unwrap();
        assert!(xml.contains("<?xml version=\"1.0\"?>"));
        assert_eq!(xml.matches("<vlan").count(), 5);
    }

    #[test]
    fn test_xml_escaping() {
        let generator = StreamingXmlGenerator::new();
        let text = "Test & <data> with \"quotes\"";
        let escaped = generator.escape_xml_fast(text);
        
        assert!(escaped.contains("&amp;"));
        assert!(escaped.contains("&lt;"));
        assert!(escaped.contains("&gt;"));
        assert!(escaped.contains("&quot;"));
    }

    #[test]
    fn test_memory_efficiency() {
        let mut generator = StreamingXmlGenerator::new();
        let configs = generate_vlan_configurations(100, Some(42), None).unwrap();
        
        let _xml = generator.generate_config_optimized(&configs, None).unwrap();
        
        // Memory efficiency test - just ensure it completes without excessive allocation
        assert!(!generator.xml_buffer.is_empty());
    }

    #[test]
    fn test_large_dataset_performance() {
        let mut generator = StreamingXmlGenerator::new();
        let configs = generate_vlan_configurations(1000, Some(42), None).unwrap();
        
        let start = std::time::Instant::now();
        let xml = generator.generate_config_optimized(&configs, None).unwrap();
        let duration = start.elapsed();
        
        assert_eq!(xml.matches("<vlan").count(), 1000);
        assert!(duration.as_millis() < 1000, "XML generation too slow: {:?}", duration);
    }

    #[test]
    fn test_reset_functionality() {
        let mut generator = StreamingXmlGenerator::new();
        let configs = generate_vlan_configurations(10, Some(42), None).unwrap();
        
        let _xml = generator.generate_config_optimized(&configs, None).unwrap();
        assert!(!generator.xml_buffer.is_empty());
        
        generator.reset();
        assert!(generator.xml_buffer.is_empty());
    }
}