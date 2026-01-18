//! Workflow entity.

use crate::value_objects::Phase;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Workflow entity.
///
/// Represents a development workflow with ordered phases and approval tracking.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workflow {
    /// Unique identifier for this workflow.
    pub id: String,
    /// Name of this workflow.
    pub name: String,
    /// Ordered list of phases in this workflow.
    pub phases: Vec<Phase>,
    /// Current phase in the workflow.
    pub current_phase: Phase,
    /// Approval status for each phase.
    pub approvals: HashMap<Phase, bool>,
}

impl Workflow {
    /// Creates a new workflow with the standard phase sequence.
    pub fn new(name: String) -> Self {
        let phases = Phase::all();
        let current_phase = phases[0];

        Self {
            id: Uuid::new_v4().to_string(),
            name,
            phases: phases.clone(),
            current_phase,
            approvals: phases.iter().map(|p| (*p, false)).collect(),
        }
    }

    /// Creates a workflow with custom phases.
    ///
    /// # Errors
    ///
    /// Returns an error if the phases list is empty.
    pub fn with_phases(name: String, phases: Vec<Phase>) -> Result<Self, crate::DomainError> {
        if phases.is_empty() {
            return Err(crate::DomainError::ValidationError(
                "Workflow must have at least one phase".to_string(),
            ));
        }

        let current_phase = phases[0];

        Ok(Self {
            id: Uuid::new_v4().to_string(),
            name,
            phases: phases.clone(),
            current_phase,
            approvals: phases.iter().map(|p| (*p, false)).collect(),
        })
    }

    /// Moves to the next phase.
    ///
    /// # Errors
    ///
    /// Returns an error if already at the last phase or if current phase is not approved.
    pub fn next_phase(&mut self) -> Result<(), crate::DomainError> {
        if !self.can_proceed() {
            return Err(crate::DomainError::ValidationError(
                "Current phase must be approved before proceeding".to_string(),
            ));
        }

        let current_index = self
            .phases
            .iter()
            .position(|p| *p == self.current_phase)
            .ok_or_else(|| {
                crate::DomainError::ValidationError(
                    "Current phase not found in workflow".to_string(),
                )
            })?;

        if current_index >= self.phases.len() - 1 {
            return Err(crate::DomainError::ValidationError(
                "Already at the last phase".to_string(),
            ));
        }

        self.current_phase = self.phases[current_index + 1];
        Ok(())
    }

    /// Approves a specific phase.
    pub fn approve_phase(&mut self, phase: Phase) {
        self.approvals.insert(phase, true);
    }

    /// Checks if the workflow can proceed to the next phase.
    pub fn can_proceed(&self) -> bool {
        self.approvals
            .get(&self.current_phase)
            .copied()
            .unwrap_or(false)
    }

    /// Gets the next phase, if any.
    pub fn peek_next_phase(&self) -> Option<Phase> {
        let current_index = self.phases.iter().position(|p| *p == self.current_phase)?;

        if current_index >= self.phases.len() - 1 {
            None
        } else {
            Some(self.phases[current_index + 1])
        }
    }

    /// Checks if this is the last phase.
    pub fn is_last_phase(&self) -> bool {
        self.peek_next_phase().is_none()
    }

    /// Gets the approval status of a phase.
    pub fn is_approved(&self, phase: Phase) -> bool {
        self.approvals.get(&phase).copied().unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_workflow_creation() {
        let workflow = Workflow::new("Standard".to_string());

        assert_eq!(workflow.name, "Standard");
        assert_eq!(workflow.current_phase, Phase::Spec);
        assert_eq!(workflow.phases.len(), 6);
        assert!(!workflow.can_proceed());
    }

    #[test]
    fn test_workflow_with_custom_phases() {
        let phases = vec![Phase::Spec, Phase::Tdd, Phase::Review];
        let workflow = Workflow::with_phases("Custom".to_string(), phases.clone()).unwrap();

        assert_eq!(workflow.name, "Custom");
        assert_eq!(workflow.phases, phases);
        assert_eq!(workflow.current_phase, Phase::Spec);
    }

    #[test]
    fn test_workflow_empty_phases_rejected() {
        let result = Workflow::with_phases("Empty".to_string(), vec![]);
        assert!(result.is_err());
    }

    #[test]
    fn test_workflow_approve_phase() {
        let mut workflow = Workflow::new("Test".to_string());

        assert!(!workflow.can_proceed());
        assert!(!workflow.is_approved(Phase::Spec));

        workflow.approve_phase(Phase::Spec);
        assert!(workflow.can_proceed());
        assert!(workflow.is_approved(Phase::Spec));
    }

    #[test]
    fn test_workflow_next_phase() {
        let mut workflow = Workflow::new("Test".to_string());

        // Cannot proceed without approval
        assert!(workflow.next_phase().is_err());

        // Approve and proceed
        workflow.approve_phase(Phase::Spec);
        workflow.next_phase().unwrap();
        assert_eq!(workflow.current_phase, Phase::Tasks);

        // Approve and proceed again
        workflow.approve_phase(Phase::Tasks);
        workflow.next_phase().unwrap();
        assert_eq!(workflow.current_phase, Phase::Tdd);
    }

    #[test]
    fn test_workflow_cannot_proceed_past_last_phase() {
        let mut workflow = Workflow::new("Test".to_string());

        // Move to last phase
        for phase in Phase::all() {
            workflow.approve_phase(phase);
            if !workflow.is_last_phase() {
                workflow.next_phase().unwrap();
            }
        }

        assert_eq!(workflow.current_phase, Phase::Merge);
        assert!(workflow.is_last_phase());

        // Try to proceed past last phase
        workflow.approve_phase(Phase::Merge);
        assert!(workflow.next_phase().is_err());
    }

    #[test]
    fn test_workflow_peek_next_phase() {
        let workflow = Workflow::new("Test".to_string());

        assert_eq!(workflow.current_phase, Phase::Spec);
        assert_eq!(workflow.peek_next_phase(), Some(Phase::Tasks));
    }

    #[test]
    fn test_workflow_is_last_phase() {
        let mut workflow = Workflow::new("Test".to_string());

        assert!(!workflow.is_last_phase());

        // Move to last phase
        for phase in Phase::all() {
            workflow.approve_phase(phase);
            if !workflow.is_last_phase() {
                workflow.next_phase().unwrap();
            }
        }

        assert!(workflow.is_last_phase());
    }

    #[test]
    fn test_workflow_clone() {
        let workflow = Workflow::new("Test".to_string());
        let cloned = workflow.clone();

        assert_eq!(workflow.id, cloned.id);
        assert_eq!(workflow.name, cloned.name);
        assert_eq!(workflow.current_phase, cloned.current_phase);
    }
}
