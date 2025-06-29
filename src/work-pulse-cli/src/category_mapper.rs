use once_cell::sync::Lazy;
use std::collections::HashMap;

/// A static category mapping table that maps category names to standardized categories
/// This is initialized once and can be accessed globally throughout the application
pub static CATEGORY_MAP: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut map = HashMap::new();
    
    map.insert("CurrentVersion", "Current Version");
    map.insert("NextVersion", "Next Version");
    map.insert("SWATrainer", "SWA Trainer");
    map.insert("Sonstiges", "Other");
    map.insert("TechnoCluster", "TC: SW-Defined Innovation");
    
    map
});

/// Maps a category name to its standardized form.
/// 
/// # Arguments
/// 
/// - `category`: The category name to map, case-insensitive.
/// 
/// Returns the mapped category if found, otherwise returns the original input
pub fn map_category(category: &str) -> Option<&str> {
    CATEGORY_MAP.get(category).copied()
}

/// Gets all available category mappings as a vector of (input, output) pairs.
pub fn get_all_mappings() -> Vec<(&'static str, &'static str)> {
    CATEGORY_MAP.iter().map(|(&k, &v)| (k, v)).collect()
}

/// Gets all unique standardized categories.
pub fn get_standard_categories() -> Vec<&'static str> {
    let mut categories: Vec<_> = CATEGORY_MAP.values().cloned().collect();
    categories.sort();
    categories.dedup();
    categories
}

/// Checks if a category name has a mapping.
pub fn has_mapping(category: &str) -> bool {
    CATEGORY_MAP.contains_key(category)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_category_should_return_mapped_category_if_mapping_exists() {
        assert_eq!(map_category("CurrentVersion"), Some("Current Version"));
        assert_eq!(map_category("NextVersion"), Some("Next Version"));
        assert_eq!(map_category("SWATrainer"), Some("SWA Trainer"));
        assert_eq!(map_category("Sonstiges"), Some("Other"));
        assert_eq!(map_category("TechnoCluster"), Some("TC: SW-Defined Innovation"));
    }

    #[test]
    fn map_category_should_return_none_if_no_mapping_exists() {
        assert_eq!(map_category("nonexistent"), None);
    }

    #[test]
    fn get_all_mappings_should_return_all_mappings() {
        let mappings = get_all_mappings();

        assert_eq!(mappings.len(), 5);

        assert!(mappings.contains(&("CurrentVersion", "Current Version")));
        assert!(mappings.contains(&("NextVersion", "Next Version")));
        assert!(mappings.contains(&("SWATrainer", "SWA Trainer")));
        assert!(mappings.contains(&("Sonstiges", "Other")));
        assert!(mappings.contains(&("TechnoCluster", "TC: SW-Defined Innovation")));
    }

    #[test]
    fn get_standard_categories_should_return_unique_categories() {
        let categories = get_standard_categories();

        assert_eq!(categories.len(), 5);

        assert!(categories.contains(&"Current Version"));
        assert!(categories.contains(&"Next Version"));
        assert!(categories.contains(&"SWA Trainer"));
        assert!(categories.contains(&"Other"));
        assert!(categories.contains(&"TC: SW-Defined Innovation"));
    }

    #[test]
    fn has_mapping_should_return_true_if_mapping_exists() {
        assert!(has_mapping("CurrentVersion"));
    }

    #[test]
    fn has_mapping_should_return_false_if_mapping_does_not_exist() {
        assert!(!has_mapping("nonexistent"));
    }
}
