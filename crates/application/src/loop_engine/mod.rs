//! Loop engine module for autonomous task execution.
//!
//! This module provides components for detecting task completion and managing
//! autonomous execution loops.

pub mod completion_detector;
pub mod engine;

pub use completion_detector::CompletionDetector;
pub use engine::LoopEngine;
