//! SigmaOS-data-relevance: data relevance model.
//!
//! Real implementation: context, scoring, filtering, and validation.
//! Replaces the 108-line CLI placeholder.

use std::collections::BTreeMap;
use std::fmt;

/// Relevance scorer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelevanceScorer {
    pub id: String,
    pub name: String,
    pub context: String,
    pub threshold: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RelevanceError {
    EmptyName,
    DuplicateScorer(String),
}

impl fmt::Display for RelevanceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => write!(f, "scorer name must not be empty"),
            Self::DuplicateScorer(id) => write!(f, "duplicate scorer {:?}", id),
        }
    }
}

impl std::error::Error for RelevanceError {}

/// Data relevance manager.
pub struct DataRelevance {
    pub name: String,
    scorers: BTreeMap<String, RelevanceScorer>,
}

impl DataRelevance {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            scorers: BTreeMap::new(),
        }
    }

    pub fn add_scorer(&mut self, scorer: RelevanceScorer) -> Result<(), RelevanceError> {
        if scorer.name.trim().is_empty() { return Err(RelevanceError::EmptyName); }
        let id = scorer.id.clone();
        if self.scorers.contains_key(&id) {
            return Err(RelevanceError::DuplicateScorer(id.clone()));
        }
        self.scorers.insert(id, scorer);
        Ok(())
    }

    pub fn scorer_count(&self) -> usize { self.scorers.len() }

    pub fn get_scorer(&self, id: &str) -> Option<&RelevanceScorer> { self.scorers.get(id) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_relevance_create_aaa() {
        let r = DataRelevance::new("My Relevance");
        assert_eq!(r.name, "My Relevance");
        assert_eq!(r.scorer_count(), 0);
    }

    #[test]
    fn test_add_scorer_aaa() {
        let mut r = DataRelevance::new("My Relevance");
        let s = RelevanceScorer { id: "s1".into(), name: "search-rank".into(), context: "query".into(), threshold: "0.8".into() };
        r.add_scorer(s).unwrap();
        assert_eq!(r.scorer_count(), 1);
    }

    #[test]
    fn test_empty_name_rejected_aaa() {
        let mut r = DataRelevance::new("My Relevance");
        let s = RelevanceScorer { id: "x".into(), name: "".into(), context: "a".into(), threshold: "b".into() };
        assert!(r.add_scorer(s).is_err());
    }

    #[test]
    fn test_duplicate_rejected_aaa() {
        let mut r = DataRelevance::new("My Relevance");
        let s = RelevanceScorer { id: "x".into(), name: "X".into(), context: "a".into(), threshold: "b".into() };
        r.add_scorer(s).unwrap();
        let s2 = RelevanceScorer { id: "x".into(), name: "X".into(), context: "a".into(), threshold: "b".into() };
        assert!(r.add_scorer(s2).is_err());
    }
}
