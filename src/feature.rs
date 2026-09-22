use ontolius::{Identified, TermId};

use crate::{Fraction, Observable};

/// Represents annotation to an ontology term of an individual
/// (a person, mouse, gene, etc.).
///
/// The annotation has a [`TermId`] as an identifier.
/// The annotation can be either in the present or the excluded state,
/// which indicates that it's presence has been sought for and confirmed,
/// or excluded, respectively.
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
pub struct AggregatedFeature<'a, T = u32> {
    term_id: std::borrow::Cow<'a, TermId>,
    fraction: Fraction<T>,
}

impl<T> From<(TermId, Fraction<T>)> for AggregatedFeature<'_, T> {
    fn from(value: (TermId, Fraction<T>)) -> Self {
        Self {
            term_id: std::borrow::Cow::Owned(value.0),
            fraction: value.1,
        }
    }
}

impl<'a, T> From<(&'a TermId, Fraction<T>)> for AggregatedFeature<'a, T> {
    fn from(value: (&'a TermId, Fraction<T>)) -> Self {
        Self {
            term_id: std::borrow::Cow::Borrowed(value.0),
            fraction: value.1,
        }
    }
}

impl<T> AggregatedFeature<'_, T>
where
    T: Clone,
{
    /// Get the fraction of individuals the feature was observed in.
    ///
    /// ```
    /// use ontolius::TermId;
    /// use phenotypes::Fraction;
    /// use phenotypes::feature::AggregatedFeature;
    ///
    /// let t: TermId = "HP:0001250".parse().expect("CURIE is valid");
    /// let fr: Fraction<u8> = (2, 4).try_into().expect("Counts are valid");
    /// let f = AggregatedFeature::from((t, fr));
    ///
    /// assert_eq!(f.fraction().n(), 2);
    /// assert_eq!(f.fraction().m(), 4);
    /// ```
    pub fn fraction(&self) -> Fraction<T> {
        Clone::clone(&self.fraction)
    }
}

impl<T> Identified for AggregatedFeature<'_, T> {
    fn identifier(&self) -> &TermId {
        &self.term_id
    }
}

impl<T> Observable for AggregatedFeature<'_, T>
where
    T: PartialOrd<u8> + Clone,
{
    fn is_present(&self) -> bool {
        self.fraction.n() > 0
    }
}
