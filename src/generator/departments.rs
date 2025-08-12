//! Department name constants for realistic VLAN naming

/// Standard department names for generating realistic VLAN descriptions
pub const DEPARTMENTS: &[&str] = &[
    "Sales",
    "IT",
    "HR",
    "Finance",
    "Marketing",
    "Operations",
    "Engineering",
    "Support",
    "Legal",
    "Procurement",
    "Security",
    "Development",
    "QA",
    "Research",
    "Training",
    "Management",
    "Accounting",
    "Customer Service",
    "Logistics",
    "Production",
];

/// Get a random department name using the provided RNG
pub fn random_department<R: rand::Rng>(rng: &mut R) -> &'static str {
    DEPARTMENTS[rng.gen_range(0..DEPARTMENTS.len())]
}

/// Get the number of available departments
pub fn department_count() -> usize {
    DEPARTMENTS.len()
}

/// Get all department names as a slice
pub fn all_departments() -> &'static [&'static str] {
    DEPARTMENTS
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    #[test]
    fn test_department_constants() {
        // Test that we have departments defined
        assert!(DEPARTMENTS.len() >= 16); // At least the original departments

        // Verify specific departments from the issue are included
        assert!(DEPARTMENTS.contains(&"Sales"));
        assert!(DEPARTMENTS.contains(&"IT"));
        assert!(DEPARTMENTS.contains(&"Engineering"));
        assert!(DEPARTMENTS.contains(&"Security"));
    }

    #[test]
    fn test_department_count() {
        assert_eq!(department_count(), DEPARTMENTS.len());
        assert!(department_count() > 0);
    }

    #[test]
    fn test_all_departments() {
        let all = all_departments();
        assert_eq!(all.len(), DEPARTMENTS.len());
        assert_eq!(all, DEPARTMENTS);
    }

    #[test]
    fn test_random_department() {
        let mut rng = ChaCha8Rng::seed_from_u64(42);

        // Generate several random departments
        for _ in 0..20 {
            let dept = random_department(&mut rng);
            assert!(DEPARTMENTS.contains(&dept));
            assert!(!dept.is_empty());
        }
    }

    #[test]
    fn test_random_department_deterministic() {
        let mut rng1 = ChaCha8Rng::seed_from_u64(12345);
        let mut rng2 = ChaCha8Rng::seed_from_u64(12345);

        // Same seed should produce same sequence
        for _ in 0..10 {
            let dept1 = random_department(&mut rng1);
            let dept2 = random_department(&mut rng2);
            assert_eq!(dept1, dept2);
        }
    }

    #[test]
    fn test_departments_are_valid() {
        for dept in DEPARTMENTS {
            assert!(!dept.is_empty());
            assert!(!dept.contains('\n'));
            assert!(!dept.contains('\t'));
            // Departments should be reasonable length
            assert!(dept.len() <= 50);
        }
    }
}
