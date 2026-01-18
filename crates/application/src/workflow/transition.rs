//! Workflow state transition logic.

use crate::error::{ApplicationError, Result};
use domain::entities::Workflow;
use domain::value_objects::Phase;

/// Checks if a transition from one phase to another is valid.
///
/// Valid transitions follow the standard workflow:
/// SPEC → TASKS → TDD → REVIEW → RETRO → MERGE
///
/// # Examples
///
/// ```
/// use application::workflow::transition::can_transition;
/// use domain::value_objects::Phase;
///
/// assert!(can_transition(Phase::Spec, Phase::Tasks));
/// assert!(!can_transition(Phase::Spec, Phase::Review));
/// ```
pub fn can_transition(from: Phase, to: Phase) -> bool {
    match (from, to) {
        (Phase::Spec, Phase::Tasks) => true,
        (Phase::Tasks, Phase::Tdd) => true,
        (Phase::Tdd, Phase::Review) => true,
        (Phase::Review, Phase::Retro) => true,
        (Phase::Retro, Phase::Merge) => true,
        // Same phase is allowed (no-op)
        (a, b) if a == b => true,
        // All other transitions are invalid
        _ => false,
    }
}

/// Gets the next phase in the standard workflow.
///
/// Returns `None` if already at the last phase (MERGE).
///
/// # Examples
///
/// ```
/// use application::workflow::transition::next_phase;
/// use domain::value_objects::Phase;
///
/// assert_eq!(next_phase(Phase::Spec), Some(Phase::Tasks));
/// assert_eq!(next_phase(Phase::Merge), None);
/// ```
pub fn next_phase(current: Phase) -> Option<Phase> {
    current.next()
}

/// Performs a phase transition on a workflow.
///
/// # Errors
///
/// Returns an error if:
/// - The transition is not valid
/// - The current phase is not approved
/// - The workflow is already at the target phase
///
/// # Examples
///
/// ```
/// use application::workflow::transition::transition;
/// use domain::entities::Workflow;
/// use domain::value_objects::Phase;
///
/// let mut workflow = Workflow::new("Test".to_string());
/// workflow.approve_phase(Phase::Spec);
///
/// transition(&mut workflow, Phase::Tasks).unwrap();
/// assert_eq!(workflow.current_phase, Phase::Tasks);
/// ```
pub fn transition(workflow: &mut Workflow, to: Phase) -> Result<()> {
    let from = workflow.current_phase;

    // Check if transition is valid
    if !can_transition(from, to) {
        return Err(ApplicationError::WorkflowTransition(format!(
            "不正なフェーズ遷移: {} → {}",
            from, to
        )));
    }

    // If already at target phase, it's a no-op
    if from == to {
        return Ok(());
    }

    // Check if current phase is approved
    if !workflow.is_approved(from) {
        return Err(ApplicationError::WorkflowTransition(format!(
            "フェーズ {} はまだ承認されていません",
            from.japanese_name()
        )));
    }

    // Update workflow to next phase
    workflow.next_phase().map_err(|e| {
        ApplicationError::WorkflowTransition(format!("フェーズ遷移に失敗しました: {}", e))
    })?;

    Ok(())
}

/// Automatically transitions to the next phase if approved.
///
/// This is a convenience function that combines approval checking
/// and transitioning to the next phase.
///
/// # Errors
///
/// Returns an error if:
/// - The current phase is not approved
/// - Already at the last phase
pub fn auto_transition(workflow: &mut Workflow) -> Result<()> {
    if !workflow.can_proceed() {
        return Err(ApplicationError::WorkflowTransition(format!(
            "フェーズ {} はまだ承認されていません",
            workflow.current_phase.japanese_name()
        )));
    }

    let next = workflow
        .peek_next_phase()
        .ok_or_else(|| ApplicationError::WorkflowTransition("既に最終フェーズです".to_string()))?;

    transition(workflow, next)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_transition_valid() {
        assert!(can_transition(Phase::Spec, Phase::Tasks));
        assert!(can_transition(Phase::Tasks, Phase::Tdd));
        assert!(can_transition(Phase::Tdd, Phase::Review));
        assert!(can_transition(Phase::Review, Phase::Retro));
        assert!(can_transition(Phase::Retro, Phase::Merge));
    }

    #[test]
    fn test_can_transition_same_phase() {
        assert!(can_transition(Phase::Spec, Phase::Spec));
        assert!(can_transition(Phase::Tasks, Phase::Tasks));
    }

    #[test]
    fn test_can_transition_invalid() {
        assert!(!can_transition(Phase::Spec, Phase::Review));
        assert!(!can_transition(Phase::Spec, Phase::Tdd));
        assert!(!can_transition(Phase::Tasks, Phase::Review));
        assert!(!can_transition(Phase::Merge, Phase::Spec));
    }

    #[test]
    fn test_next_phase() {
        assert_eq!(next_phase(Phase::Spec), Some(Phase::Tasks));
        assert_eq!(next_phase(Phase::Tasks), Some(Phase::Tdd));
        assert_eq!(next_phase(Phase::Tdd), Some(Phase::Review));
        assert_eq!(next_phase(Phase::Review), Some(Phase::Retro));
        assert_eq!(next_phase(Phase::Retro), Some(Phase::Merge));
        assert_eq!(next_phase(Phase::Merge), None);
    }

    #[test]
    fn test_transition_success() {
        let mut workflow = Workflow::new("Test".to_string());

        // Approve and transition
        workflow.approve_phase(Phase::Spec);
        let result = transition(&mut workflow, Phase::Tasks);
        assert!(result.is_ok());
        assert_eq!(workflow.current_phase, Phase::Tasks);
    }

    #[test]
    fn test_transition_not_approved() {
        let mut workflow = Workflow::new("Test".to_string());

        // Try to transition without approval
        let result = transition(&mut workflow, Phase::Tasks);
        assert!(result.is_err());
        assert_eq!(workflow.current_phase, Phase::Spec);
    }

    #[test]
    fn test_transition_invalid() {
        let mut workflow = Workflow::new("Test".to_string());
        workflow.approve_phase(Phase::Spec);

        // Try invalid transition
        let result = transition(&mut workflow, Phase::Review);
        assert!(result.is_err());
        assert_eq!(workflow.current_phase, Phase::Spec);
    }

    #[test]
    fn test_transition_same_phase_noop() {
        let mut workflow = Workflow::new("Test".to_string());
        workflow.approve_phase(Phase::Spec);

        let result = transition(&mut workflow, Phase::Spec);
        assert!(result.is_ok());
        assert_eq!(workflow.current_phase, Phase::Spec);
    }

    #[test]
    fn test_auto_transition_success() {
        let mut workflow = Workflow::new("Test".to_string());

        workflow.approve_phase(Phase::Spec);
        let result = auto_transition(&mut workflow);
        assert!(result.is_ok());
        assert_eq!(workflow.current_phase, Phase::Tasks);
    }

    #[test]
    fn test_auto_transition_not_approved() {
        let mut workflow = Workflow::new("Test".to_string());

        let result = auto_transition(&mut workflow);
        assert!(result.is_err());
    }

    #[test]
    fn test_auto_transition_at_last_phase() {
        let mut workflow = Workflow::new("Test".to_string());

        // Move to last phase
        for phase in Phase::all() {
            workflow.approve_phase(phase);
            if !workflow.is_last_phase() {
                workflow.next_phase().unwrap();
            }
        }

        assert_eq!(workflow.current_phase, Phase::Merge);

        // Try to auto-transition from last phase
        let result = auto_transition(&mut workflow);
        assert!(result.is_err());
    }
}
