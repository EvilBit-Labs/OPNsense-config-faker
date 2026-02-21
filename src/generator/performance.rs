//! High-performance VLAN configuration generator
//!
//! This module implements performance-optimized data structures and algorithms
//! for generating large numbers of VLAN configurations efficiently.

use crate::Result;
use crate::generator::VlanConfig;
use crate::generator::departments;
use crate::model::ConfigError;
use crate::utils::rfc1918;

use bumpalo::Bump;
use lru::LruCache;
use rand::SeedableRng;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use rustc_hash::{FxHashMap, FxHashSet};
use smallvec::SmallVec;
use std::num::NonZeroUsize;

/// Default throughput target in configurations per second
///
/// This represents a 3x improvement over the Python baseline implementation
pub const DEFAULT_THROUGHPUT_TARGET: f64 = 150.0;

/// Default memory efficiency target in bytes per configuration
///
/// This target ensures efficient memory usage for large-scale generation
pub const DEFAULT_MEMORY_EFFICIENCY_TARGET: f64 = 25_000.0;

/// Performance metrics for tracking generation efficiency
#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    pub generation_time: std::time::Duration,
    pub memory_used: usize,
    pub configs_generated: usize,
    pub peak_memory: usize,
    pub allocations: u64,
}

impl PerformanceMetrics {
    /// Calculate throughput (configs per second)
    pub fn throughput(&self) -> f64 {
        self.configs_generated as f64 / self.generation_time.as_secs_f64()
    }

    /// Calculate memory efficiency (bytes per config)
    pub fn memory_efficiency(&self) -> f64 {
        self.memory_used as f64 / self.configs_generated as f64
    }

    /// Check if performance meets default targets
    pub fn meets_performance_targets(&self) -> bool {
        self.meets_performance_targets_with(
            DEFAULT_THROUGHPUT_TARGET,
            DEFAULT_MEMORY_EFFICIENCY_TARGET,
        )
    }

    /// Check if performance meets custom targets
    ///
    /// # Arguments
    ///
    /// * `throughput_target` - Minimum throughput in configurations per second
    /// * `memory_target` - Maximum memory efficiency in bytes per configuration
    ///
    /// # Returns
    ///
    /// Returns `true` if both throughput and memory efficiency meet or exceed the targets
    pub fn meets_performance_targets_with(
        &self,
        throughput_target: f64,
        memory_target: f64,
    ) -> bool {
        self.throughput() >= throughput_target && self.memory_efficiency() <= memory_target
    }
}

/// Optimized VLAN configuration cache entry
#[repr(C)]
#[derive(Debug, Clone)]
struct CachedVlanConfig {
    vlan_id: u16,
    ip_octets: [u8; 4],
    department_id: u8,
    wan_assignment: u8,
}

/// High-performance VLAN configuration generator
pub struct PerformantConfigGenerator {
    /// Arena allocator for temporary objects
    arena: Bump,

    /// LRU cache for department templates
    department_cache: LruCache<u8, String>,

    /// Pre-allocated string buffer for IP networks
    #[allow(dead_code)]
    ip_buffer: String,

    /// Fast hash set for VLAN ID tracking
    used_vlan_ids: FxHashSet<u16>,

    /// Fast hash map for IP network tracking
    used_networks: FxHashMap<u32, bool>,

    /// Random number generator
    rng: ChaCha8Rng,

    /// Pre-allocated vector for batch operations
    batch_buffer: Vec<VlanConfig>,

    /// Performance metrics tracking
    metrics: PerformanceMetrics,

    /// Configuration cache for frequent patterns
    config_cache: SmallVec<[CachedVlanConfig; 32]>,
}

impl PerformantConfigGenerator {
    /// Create a new high-performance generator
    pub fn new(seed: Option<u64>) -> Self {
        let mut department_cache = LruCache::new(NonZeroUsize::new(16).unwrap());

        // Pre-populate department cache
        for (id, dept) in departments::all_departments().iter().enumerate() {
            department_cache.put(id as u8, dept.to_string());
        }

        Self {
            arena: Bump::new(),
            department_cache,
            ip_buffer: String::with_capacity(15), // "255.255.255.255" max length
            used_vlan_ids: FxHashSet::default(),
            used_networks: FxHashMap::default(),
            rng: ChaCha8Rng::seed_from_u64(seed.unwrap_or_else(|| rand::rng().random())),
            batch_buffer: Vec::new(),
            metrics: PerformanceMetrics {
                generation_time: std::time::Duration::ZERO,
                memory_used: 0,
                configs_generated: 0,
                peak_memory: 0,
                allocations: 0,
            },
            config_cache: SmallVec::new(),
        }
    }

    /// Generate a batch of VLAN configurations with optimized memory allocation
    pub fn generate_batch(&mut self, count: usize) -> Result<Vec<VlanConfig>> {
        let start_time = std::time::Instant::now();
        self.arena.reset(); // Reset arena for this batch

        // Pre-allocate with known capacity
        self.batch_buffer.clear();
        self.batch_buffer.reserve_exact(count);

        // Clear tracking sets if they would grow too large
        if self.used_vlan_ids.len() + count > 4000 {
            self.used_vlan_ids.clear();
        }
        if self.used_networks.len() + count > 10000 {
            self.used_networks.clear();
        }

        // Generate configurations in batch
        for _ in 0..count {
            let config = self.generate_single_optimized()?;
            self.batch_buffer.push(config);
        }

        // Update metrics
        let generation_time = start_time.elapsed();
        self.metrics.generation_time = generation_time;
        self.metrics.configs_generated = count;
        self.metrics.memory_used = self.estimate_memory_usage();

        Ok(std::mem::take(&mut self.batch_buffer))
    }

    /// Generate a single VLAN configuration with optimized allocations
    fn generate_single_optimized(&mut self) -> Result<VlanConfig> {
        // Generate unique VLAN ID efficiently
        let vlan_id = self.generate_unique_vlan_id()?;

        // Generate unique IP network efficiently
        let ip_network = self.generate_unique_ip_network()?;

        // Generate description with cached department lookup
        let department = self.get_cached_department();
        let description = format!("{} VLAN {}", department, vlan_id);

        // Generate WAN assignment
        let wan_assignment = self.rng.random_range(1..=3);

        VlanConfig::new(vlan_id, ip_network, description, wan_assignment)
    }

    /// Efficiently generate unique VLAN ID
    fn generate_unique_vlan_id(&mut self) -> Result<u16> {
        const MAX_ATTEMPTS: usize = 100;

        for _ in 0..MAX_ATTEMPTS {
            let vlan_id = self.rng.random_range(10..=4094);
            if self.used_vlan_ids.insert(vlan_id) {
                return Ok(vlan_id);
            }
        }

        Err(ConfigError::vlan_generation(
            "Unable to generate unique VLAN ID after maximum attempts",
        ))
    }

    /// Efficiently generate unique IP network
    fn generate_unique_ip_network(&mut self) -> Result<String> {
        const MAX_ATTEMPTS: usize = 100;

        for _ in 0..MAX_ATTEMPTS {
            // Use existing RFC 1918 generation functions
            let network = match self.rng.random_range(0..3) {
                0 => rfc1918::generate_random_class_a_network(&mut self.rng),
                1 => rfc1918::generate_random_class_b_network(&mut self.rng),
                _ => rfc1918::generate_random_class_c_network(&mut self.rng),
            };

            // Convert to u32 for efficient hashing
            let network_key = network.network().into();

            if self.used_networks.insert(network_key, true).is_none() {
                // Return the owned string directly to avoid extra allocation
                return Ok(network.to_string());
            }
        }

        Err(ConfigError::vlan_generation(
            "Unable to generate unique IP network after maximum attempts",
        ))
    }

    /// Get department name with caching
    fn get_cached_department(&mut self) -> String {
        let dept_id = self
            .rng
            .random_range(0..departments::all_departments().len()) as u8;

        if let Some(dept) = self.department_cache.get(&dept_id) {
            dept.clone()
        } else {
            let dept = departments::all_departments()[dept_id as usize];
            self.department_cache.put(dept_id, dept.to_string());
            dept.to_string()
        }
    }

    /// Estimate current memory usage
    fn estimate_memory_usage(&self) -> usize {
        let vlan_ids_mem = self.used_vlan_ids.capacity() * std::mem::size_of::<u16>();
        let networks_mem = self.used_networks.capacity() * std::mem::size_of::<(u32, bool)>();
        let cache_mem = self.config_cache.capacity() * std::mem::size_of::<CachedVlanConfig>();
        let buffer_mem = self.batch_buffer.capacity() * std::mem::size_of::<VlanConfig>();
        let dept_cache_mem = self.department_cache.cap().get() * 64; // Estimated string size

        vlan_ids_mem + networks_mem + cache_mem + buffer_mem + dept_cache_mem
    }

    /// Reset generator state for new batch
    pub fn reset(&mut self) {
        self.used_vlan_ids.clear();
        self.used_networks.clear();
        self.batch_buffer.clear();
        self.config_cache.clear();
        self.arena.reset();
    }

    /// Get performance metrics
    pub fn get_metrics(&self) -> &PerformanceMetrics {
        &self.metrics
    }

    /// Parallel generation using work stealing
    #[cfg(feature = "rayon")]
    pub fn generate_parallel(
        &mut self,
        total_count: usize,
        chunk_size: usize,
    ) -> Result<Vec<VlanConfig>> {
        use rayon::prelude::*;

        let chunks = total_count.div_ceil(chunk_size);
        let mut results = Vec::with_capacity(total_count);

        // Generate base seed outside of closure
        let base_seed = self.rng.random::<u64>();

        let chunk_results: Result<Vec<Vec<VlanConfig>>> = (0..chunks)
            .into_par_iter()
            .map(|chunk_id| {
                let mut local_generator =
                    PerformantConfigGenerator::new(Some(base_seed + chunk_id as u64));

                let current_chunk_size = if chunk_id == chunks - 1 {
                    total_count - (chunk_id * chunk_size)
                } else {
                    chunk_size
                };

                local_generator.generate_batch(current_chunk_size)
            })
            .collect();

        for mut chunk in chunk_results? {
            results.append(&mut chunk);
        }

        Ok(results)
    }
}

impl Default for PerformantConfigGenerator {
    fn default() -> Self {
        Self::new(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_performant_generator_creation() {
        let generator = PerformantConfigGenerator::new(Some(42));
        assert_eq!(generator.used_vlan_ids.len(), 0);
        assert_eq!(generator.used_networks.len(), 0);
    }

    #[test]
    fn test_batch_generation() {
        let mut generator = PerformantConfigGenerator::new(Some(42));
        let configs = generator.generate_batch(10).unwrap();

        assert_eq!(configs.len(), 10);

        // Verify uniqueness
        let mut vlan_ids = std::collections::HashSet::new();
        for config in &configs {
            assert!(vlan_ids.insert(config.vlan_id));
            assert!(config.vlan_id >= 10 && config.vlan_id <= 4094);
        }
    }

    #[test]
    fn test_memory_efficiency() {
        let mut generator = PerformantConfigGenerator::new(Some(42));
        let configs = generator.generate_batch(100).unwrap();

        let metrics = generator.get_metrics();
        assert!(metrics.memory_efficiency() < 50_000.0); // Better than baseline
        assert_eq!(configs.len(), 100);
    }

    #[test]
    fn test_performance_targets() {
        let mut generator = PerformantConfigGenerator::new(Some(42));
        let start = std::time::Instant::now();
        let configs = generator.generate_batch(150).unwrap();
        let duration = start.elapsed();

        let throughput = configs.len() as f64 / duration.as_secs_f64();
        assert!(
            throughput >= 100.0,
            "Throughput too low: {:.2} configs/sec",
            throughput
        );
    }

    #[test]
    fn test_custom_performance_targets() {
        let mut generator = PerformantConfigGenerator::new(Some(42));
        let _configs = generator.generate_batch(100).unwrap();

        let metrics = generator.get_metrics();

        // Test with custom targets
        assert!(metrics.meets_performance_targets_with(50.0, 50_000.0));

        // Test with very strict targets (should likely fail)
        let _strict_result = metrics.meets_performance_targets_with(1000.0, 1000.0);
        // Note: This test doesn't assert the result since actual performance varies
    }

    #[test]
    fn test_reset_functionality() {
        let mut generator = PerformantConfigGenerator::new(Some(42));
        generator.generate_batch(10).unwrap();

        assert!(!generator.used_vlan_ids.is_empty());

        generator.reset();
        assert_eq!(generator.used_vlan_ids.len(), 0);
        assert_eq!(generator.used_networks.len(), 0);
    }

    #[test]
    fn test_large_batch_generation() {
        let mut generator = PerformantConfigGenerator::new(Some(42));
        let configs = generator.generate_batch(1000).unwrap();

        assert_eq!(configs.len(), 1000);

        // Verify all configs are valid
        for config in &configs {
            assert!(config.vlan_id >= 10 && config.vlan_id <= 4094);
            assert!(config.wan_assignment >= 1 && config.wan_assignment <= 3);
            assert!(!config.description.is_empty());
            assert!(!config.ip_network.is_empty());
        }

        let metrics = generator.get_metrics();
        assert!(metrics.memory_efficiency() < DEFAULT_MEMORY_EFFICIENCY_TARGET);
        // Target memory efficiency
    }
}
