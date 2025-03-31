//! An experimental module with example implementations.
use ontolius::{Identified, TermId};

use crate::{Fraction, Observable};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimplePhenotypicFeature {
    identifier: TermId,
    fraction: Fraction,
}

impl SimplePhenotypicFeature {
    pub fn new(identifier: TermId, fraction: Fraction) -> Self {
        Self {
            identifier,
            fraction,
        }
    }
}

impl Identified for SimplePhenotypicFeature {
    fn identifier(&self) -> &TermId {
        &self.identifier
    }
}

impl Observable for SimplePhenotypicFeature {
    fn is_present(&self) -> bool {
        self.fraction.n() > 0
    }
}
