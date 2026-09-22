//! Annotations to an ontology term.
//!
//! The module contains structures to represent information about the annotation to an ontology term.
//! The [`IndividualFeature`] represents a term annotation in the context of an individual (a person, mouse, gene, etc.).
//! The [`AggregatedFeature`] represents combines the annotation ascertained from a group of individuals, including the annotation's frequency.
use ontolius::{Identified, TermId};

use crate::{Fraction, Observable, observation::Fractional};

/// Represents annotation to an ontology term of an individual
/// (a person, mouse, gene, etc.).
///
/// The annotation has a [`TermId`] as an identifier
/// and the annotation can be either in the present or the excluded state.
/// The state indicates if the presence of the annotation has been sought for and confirmed
/// or explicitly ruled out.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IndividualFeature<'a> {
    term_id: std::borrow::Cow<'a, TermId>,
    is_present: bool,
}

impl From<(TermId, bool)> for IndividualFeature<'_> {
    fn from(value: (TermId, bool)) -> Self {
        Self {
            term_id: std::borrow::Cow::Owned(value.0),
            is_present: value.1,
        }
    }
}

impl<'a> From<(&'a TermId, bool)> for IndividualFeature<'a> {
    fn from(value: (&'a TermId, bool)) -> Self {
        Self {
            term_id: std::borrow::Cow::Borrowed(value.0),
            is_present: value.1,
        }
    }
}

impl Identified for IndividualFeature<'_> {
    fn identifier(&self) -> &TermId {
        &self.term_id
    }
}

impl Observable for IndividualFeature<'_> {
    fn is_present(&self) -> bool {
        self.is_present
    }
}

/// Represents an annotation to an ontology term aggregated over a group of individuals
/// (a cohort of human study subjects or mice, a study set of genes, etc.).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AggregatedFeature<'a, C> {
    term_id: std::borrow::Cow<'a, TermId>,
    fraction: Fraction<C>,
}

impl<C> From<(TermId, Fraction<C>)> for AggregatedFeature<'_, C> {
    fn from(value: (TermId, Fraction<C>)) -> Self {
        Self {
            term_id: std::borrow::Cow::Owned(value.0),
            fraction: value.1,
        }
    }
}

impl<'a, C> From<(&'a TermId, Fraction<C>)> for AggregatedFeature<'a, C> {
    fn from(value: (&'a TermId, Fraction<C>)) -> Self {
        Self {
            term_id: std::borrow::Cow::Borrowed(value.0),
            fraction: value.1,
        }
    }
}

impl<C> Identified for AggregatedFeature<'_, C> {
    fn identifier(&self) -> &TermId {
        &self.term_id
    }
}

impl<C> Fractional for AggregatedFeature<'_, C>
where
    C: Clone + Into<u64>,
{
    fn n(&self) -> u64 {
        self.fraction.n().into()
    }

    fn m(&self) -> u64 {
        self.fraction.m().into()
    }
}

impl<C> Observable for AggregatedFeature<'_, C>
where
    C: Clone + Into<u64>,
{
    fn is_present(&self) -> bool {
        self.n() > 0
    }
}
