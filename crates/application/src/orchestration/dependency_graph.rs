//! Dependency graph for managing Spec execution order.

use crate::error::{ApplicationError, Result};
use std::collections::{HashMap, HashSet, VecDeque};

/// Type alias for Spec identifiers.
pub type SpecId = String;

/// Manages dependencies between Specs to determine execution order.
///
/// The dependency graph tracks which Specs depend on which other Specs,
/// and provides algorithms for:
/// - Topological sorting (execution order)
/// - Cycle detection (circular dependencies)
/// - Parallel group identification (specs that can run concurrently)
#[derive(Debug, Clone)]
pub struct DependencyGraph {
    /// Maps each Spec to the Specs it depends on.
    ///
    /// For example: {"SPEC-002": ["SPEC-001"]} means SPEC-002 depends on SPEC-001.
    dependencies: HashMap<SpecId, Vec<SpecId>>,
}

impl DependencyGraph {
    /// Creates a new empty dependency graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use application::orchestration::DependencyGraph;
    ///
    /// let graph = DependencyGraph::new();
    /// ```
    pub fn new() -> Self {
        Self {
            dependencies: HashMap::new(),
        }
    }

    /// Adds a dependency relationship.
    ///
    /// # Arguments
    ///
    /// * `spec_id` - The Spec that has a dependency
    /// * `depends_on` - The Spec that must be completed first
    ///
    /// # Returns
    ///
    /// `Ok(())` if successful, or an error if adding this dependency would create a cycle.
    ///
    /// # Examples
    ///
    /// ```
    /// use application::orchestration::DependencyGraph;
    ///
    /// let mut graph = DependencyGraph::new();
    /// graph.add_dependency("SPEC-002", "SPEC-001").unwrap();
    /// ```
    pub fn add_dependency(&mut self, spec_id: &str, depends_on: &str) -> Result<()> {
        let spec_id = spec_id.to_string();
        let depends_on = depends_on.to_string();

        // Add the dependency
        self.dependencies
            .entry(spec_id.clone())
            .or_default()
            .push(depends_on.clone());

        // Ensure the depended-on spec exists in the graph
        self.dependencies.entry(depends_on.clone()).or_default();

        // Check for cycles after adding
        if let Some(cycle) = self.detect_cycle() {
            // Rollback: remove the dependency we just added
            if let Some(deps) = self.dependencies.get_mut(&spec_id) {
                deps.retain(|d| d != &depends_on);
            }
            return Err(ApplicationError::CyclicDependency(cycle));
        }

        Ok(())
    }

    /// Removes a dependency relationship.
    ///
    /// # Arguments
    ///
    /// * `spec_id` - The Spec that has the dependency
    /// * `depends_on` - The dependency to remove
    ///
    /// # Examples
    ///
    /// ```
    /// use application::orchestration::DependencyGraph;
    ///
    /// let mut graph = DependencyGraph::new();
    /// graph.add_dependency("SPEC-002", "SPEC-001").unwrap();
    /// graph.remove_dependency("SPEC-002", "SPEC-001");
    /// ```
    pub fn remove_dependency(&mut self, spec_id: &str, depends_on: &str) {
        if let Some(deps) = self.dependencies.get_mut(spec_id) {
            deps.retain(|d| d != depends_on);
        }
    }

    /// Performs topological sort to determine execution order.
    ///
    /// Returns Specs in an order where all dependencies are satisfied.
    /// Specs with no dependencies come first.
    ///
    /// # Returns
    ///
    /// A vector of Spec IDs in execution order, or an error if a cycle is detected.
    ///
    /// # Examples
    ///
    /// ```
    /// use application::orchestration::DependencyGraph;
    ///
    /// let mut graph = DependencyGraph::new();
    /// graph.add_dependency("SPEC-002", "SPEC-001").unwrap();
    /// graph.add_dependency("SPEC-003", "SPEC-001").unwrap();
    ///
    /// let order = graph.topological_sort().unwrap();
    /// assert_eq!(order[0], "SPEC-001");
    /// ```
    pub fn topological_sort(&self) -> Result<Vec<SpecId>> {
        // Check for cycles first
        if let Some(cycle) = self.detect_cycle() {
            return Err(ApplicationError::CyclicDependency(cycle));
        }

        // Calculate in-degree for each node
        let mut in_degree: HashMap<SpecId, usize> = HashMap::new();
        for spec_id in self.dependencies.keys() {
            in_degree.entry(spec_id.clone()).or_insert(0);
        }
        for deps in self.dependencies.values() {
            for dep in deps {
                *in_degree.entry(dep.clone()).or_insert(0) += 1;
            }
        }

        // Queue of nodes with in-degree 0
        let mut queue: VecDeque<SpecId> = in_degree
            .iter()
            .filter(|(_, &degree)| degree == 0)
            .map(|(spec_id, _)| spec_id.clone())
            .collect();

        let mut result = Vec::new();

        while let Some(spec_id) = queue.pop_front() {
            result.push(spec_id.clone());

            // Reduce in-degree for dependent nodes
            if let Some(deps) = self.dependencies.get(&spec_id) {
                for dep in deps {
                    if let Some(degree) = in_degree.get_mut(dep) {
                        *degree -= 1;
                        if *degree == 0 {
                            queue.push_back(dep.clone());
                        }
                    }
                }
            }
        }

        // Reverse to get correct execution order (dependencies first)
        result.reverse();

        Ok(result)
    }

    /// Detects circular dependencies in the graph.
    ///
    /// # Returns
    ///
    /// `Some(cycle)` if a cycle is found (vector of Spec IDs forming the cycle),
    /// or `None` if no cycle exists.
    ///
    /// # Examples
    ///
    /// ```
    /// use application::orchestration::DependencyGraph;
    ///
    /// let mut graph = DependencyGraph::new();
    /// graph.add_dependency("SPEC-001", "SPEC-002");
    /// // This would create a cycle, so add_dependency will fail
    /// assert!(graph.add_dependency("SPEC-002", "SPEC-001").is_err());
    /// ```
    pub fn detect_cycle(&self) -> Option<Vec<SpecId>> {
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();
        let mut path = Vec::new();

        for spec_id in self.dependencies.keys() {
            if !visited.contains(spec_id) {
                if let Some(cycle) = self.dfs_cycle_detection(
                    spec_id,
                    &mut visited,
                    &mut rec_stack,
                    &mut path,
                ) {
                    return Some(cycle);
                }
            }
        }

        None
    }

    /// DFS helper for cycle detection.
    fn dfs_cycle_detection(
        &self,
        spec_id: &str,
        visited: &mut HashSet<SpecId>,
        rec_stack: &mut HashSet<SpecId>,
        path: &mut Vec<SpecId>,
    ) -> Option<Vec<SpecId>> {
        visited.insert(spec_id.to_string());
        rec_stack.insert(spec_id.to_string());
        path.push(spec_id.to_string());

        if let Some(deps) = self.dependencies.get(spec_id) {
            for dep in deps {
                if !visited.contains(dep) {
                    if let Some(cycle) =
                        self.dfs_cycle_detection(dep, visited, rec_stack, path)
                    {
                        return Some(cycle);
                    }
                } else if rec_stack.contains(dep) {
                    // Found a cycle
                    let cycle_start = path.iter().position(|s| s == dep).unwrap();
                    return Some(path[cycle_start..].to_vec());
                }
            }
        }

        path.pop();
        rec_stack.remove(spec_id);
        None
    }

    /// Identifies groups of Specs that can run in parallel.
    ///
    /// Returns "waves" of execution where each wave contains Specs
    /// that have no dependencies on each other and can run concurrently.
    ///
    /// # Returns
    ///
    /// A vector of waves, where each wave is a vector of Spec IDs that can run in parallel.
    ///
    /// # Examples
    ///
    /// ```
    /// use application::orchestration::DependencyGraph;
    ///
    /// let mut graph = DependencyGraph::new();
    /// graph.add_dependency("SPEC-002", "SPEC-001").unwrap();
    /// graph.add_dependency("SPEC-003", "SPEC-001").unwrap();
    ///
    /// let waves = graph.get_parallel_groups().unwrap();
    /// // Wave 0: [SPEC-001]
    /// // Wave 1: [SPEC-002, SPEC-003] (can run in parallel)
    /// assert_eq!(waves[0].len(), 1);
    /// assert_eq!(waves[1].len(), 2);
    /// ```
    pub fn get_parallel_groups(&self) -> Result<Vec<Vec<SpecId>>> {
        // Check for cycles first
        if let Some(cycle) = self.detect_cycle() {
            return Err(ApplicationError::CyclicDependency(cycle));
        }

        // Calculate in-degree for reverse graph (specs that depend on current spec)
        let mut reverse_deps: HashMap<SpecId, Vec<SpecId>> = HashMap::new();
        for spec_id in self.dependencies.keys() {
            reverse_deps.entry(spec_id.clone()).or_default();
        }
        for (spec_id, deps) in &self.dependencies {
            for dep in deps {
                reverse_deps
                    .entry(dep.clone())
                    .or_default()
                    .push(spec_id.clone());
            }
        }

        // Calculate in-degree (number of dependencies)
        let mut in_degree: HashMap<SpecId, usize> = HashMap::new();
        for spec_id in self.dependencies.keys() {
            let degree = self.dependencies.get(spec_id).map(|d| d.len()).unwrap_or(0);
            in_degree.insert(spec_id.clone(), degree);
        }

        let mut waves = Vec::new();
        let mut processed = HashSet::new();

        while processed.len() < self.dependencies.len() {
            // Find all specs with in-degree 0 (no pending dependencies)
            let current_wave: Vec<SpecId> = in_degree
                .iter()
                .filter(|(spec_id, &degree)| degree == 0 && !processed.contains(*spec_id))
                .map(|(spec_id, _)| spec_id.clone())
                .collect();

            if current_wave.is_empty() {
                break; // Should not happen if cycle detection works
            }

            // Process current wave
            for spec_id in &current_wave {
                processed.insert(spec_id.clone());

                // Reduce in-degree for specs that depend on this one
                if let Some(dependents) = reverse_deps.get(spec_id) {
                    for dependent in dependents {
                        if let Some(degree) = in_degree.get_mut(dependent) {
                            *degree = degree.saturating_sub(1);
                        }
                    }
                }
            }

            waves.push(current_wave);
        }

        Ok(waves)
    }

    /// Returns the number of Specs in the graph.
    pub fn len(&self) -> usize {
        self.dependencies.len()
    }

    /// Returns true if the graph is empty.
    pub fn is_empty(&self) -> bool {
        self.dependencies.is_empty()
    }
}

impl Default for DependencyGraph {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_graph() {
        let graph = DependencyGraph::new();
        assert_eq!(graph.len(), 0);
        assert!(graph.is_empty());
    }

    #[test]
    fn test_add_dependency() {
        let mut graph = DependencyGraph::new();
        graph.add_dependency("SPEC-002", "SPEC-001").unwrap();

        assert_eq!(graph.len(), 2);
        assert!(!graph.is_empty());
    }

    #[test]
    fn test_remove_dependency() {
        let mut graph = DependencyGraph::new();
        graph.add_dependency("SPEC-002", "SPEC-001").unwrap();
        graph.remove_dependency("SPEC-002", "SPEC-001");

        // Nodes still exist, just no dependency
        assert_eq!(graph.len(), 2);
    }

    #[test]
    fn test_topological_sort_simple() {
        let mut graph = DependencyGraph::new();
        graph.add_dependency("SPEC-002", "SPEC-001").unwrap();
        graph.add_dependency("SPEC-003", "SPEC-002").unwrap();

        let order = graph.topological_sort().unwrap();
        assert_eq!(order, vec!["SPEC-001", "SPEC-002", "SPEC-003"]);
    }

    #[test]
    fn test_topological_sort_parallel() {
        let mut graph = DependencyGraph::new();
        graph.add_dependency("SPEC-002", "SPEC-001").unwrap();
        graph.add_dependency("SPEC-003", "SPEC-001").unwrap();

        let order = graph.topological_sort().unwrap();
        assert_eq!(order[0], "SPEC-001");
        // SPEC-002 and SPEC-003 can be in any order after SPEC-001
        assert!(order.contains(&"SPEC-002".to_string()));
        assert!(order.contains(&"SPEC-003".to_string()));
    }

    #[test]
    fn test_detect_cycle_simple() {
        let mut graph = DependencyGraph::new();
        graph.add_dependency("SPEC-001", "SPEC-002").unwrap();

        // This should create a cycle
        let result = graph.add_dependency("SPEC-002", "SPEC-001");
        assert!(result.is_err());
    }

    #[test]
    fn test_detect_cycle_complex() {
        let mut graph = DependencyGraph::new();
        graph.add_dependency("SPEC-002", "SPEC-001").unwrap();
        graph.add_dependency("SPEC-003", "SPEC-002").unwrap();

        // This should create a cycle: 001 -> 002 -> 003 -> 001
        let result = graph.add_dependency("SPEC-001", "SPEC-003");
        assert!(result.is_err());
    }

    #[test]
    fn test_no_cycle() {
        let mut graph = DependencyGraph::new();
        graph.add_dependency("SPEC-002", "SPEC-001").unwrap();
        graph.add_dependency("SPEC-003", "SPEC-001").unwrap();
        graph.add_dependency("SPEC-004", "SPEC-002").unwrap();

        assert!(graph.detect_cycle().is_none());
    }

    #[test]
    fn test_parallel_groups() {
        let mut graph = DependencyGraph::new();
        graph.add_dependency("SPEC-002", "SPEC-001").unwrap();
        graph.add_dependency("SPEC-003", "SPEC-001").unwrap();
        graph.add_dependency("SPEC-004", "SPEC-002").unwrap();

        let waves = graph.get_parallel_groups().unwrap();

        // Wave 0: SPEC-001
        assert_eq!(waves[0], vec!["SPEC-001"]);

        // Wave 1: SPEC-002, SPEC-003 (parallel)
        assert_eq!(waves[1].len(), 2);
        assert!(waves[1].contains(&"SPEC-002".to_string()));
        assert!(waves[1].contains(&"SPEC-003".to_string()));

        // Wave 2: SPEC-004
        assert_eq!(waves[2], vec!["SPEC-004"]);
    }

    #[test]
    fn test_parallel_groups_independent() {
        let mut graph = DependencyGraph::new();
        // Add specs with no dependencies
        graph.dependencies.insert("SPEC-001".to_string(), vec![]);
        graph.dependencies.insert("SPEC-002".to_string(), vec![]);
        graph.dependencies.insert("SPEC-003".to_string(), vec![]);

        let waves = graph.get_parallel_groups().unwrap();

        // All should be in wave 0 (no dependencies)
        assert_eq!(waves.len(), 1);
        assert_eq!(waves[0].len(), 3);
        assert!(waves[0].contains(&"SPEC-001".to_string()));
        assert!(waves[0].contains(&"SPEC-002".to_string()));
        assert!(waves[0].contains(&"SPEC-003".to_string()));
    }

    #[test]
    fn test_topological_sort_with_cycle() {
        let mut graph = DependencyGraph::new();
        graph.add_dependency("SPEC-001", "SPEC-002").unwrap();
        let result = graph.add_dependency("SPEC-002", "SPEC-001"); // Will fail

        // The second add_dependency should fail due to cycle detection
        assert!(result.is_err());

        // The graph should only contain the first dependency
        let order = graph.topological_sort().unwrap();
        assert_eq!(order, vec!["SPEC-002", "SPEC-001"]);
    }
}
