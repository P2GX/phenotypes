use std::collections::HashSet;

use ontolius::TermId;

use crate::temporal::AgeInDays;

/*
We want to model:
- term IDs
- observation status
- observation frequency
- disease model with annotations, each annotation has
  - term ID
  - frequency
- individual
  - as a bag of terms
  - as a bag of terms with status
  - as a bag of terms with observation times

- similarity measures

We assume TermIds are cheap to clone.
*/

// pub struct ObservedTermId {
//     pub term_id: TermId,
//     pub is_present: bool,
// }

// /// Create observed term id from fractional term id.

// /// ```
// /// use phenotypes::Fraction;
// /// use phenotypes::similarity::{FractionalTermId, ObservedTermId};

// /// let fti = FractionalTermId{
// ///   term_id: "HP:0001250".parse().unwrap(),
// ///   frequency: Fraction::try_from((1, 10)).unwrap(),
// /// };

// /// let oti: ObservedTermId = fti.into();
// /// assert_eq!(oti.is_present, true);
// ///```

// impl<T> From<FractionalTermId<T>> for ObservedTermId
// where
//     T: Clone + PartialOrd<u8>,
// {
//     fn from(value: FractionalTermId<T>) -> Self {
//         Self {
//             term_id: value.term_id,
//             is_present: value.frequency.n().gt(&0u8),
//         }
//     }
// }

// pub struct FractionalTermId<T = u32> {
//     pub term_id: TermId,
//     pub frequency: Fraction<T>,
// }

pub struct TemporalTermId {
    pub term_id: TermId,
    pub is_present: bool,
    pub age: AgeInDays,
}

pub struct Subject<T> {
    anns: Vec<T>,
}

/// The error documenting the reason for failure.
#[derive(Debug, Clone, PartialEq)]
pub enum SimilarityMeasureError {
    InvalidInput,
}

/// Compute similarity between items `a` and `b`.
///
/// The computation is fallible.
pub trait SimilarityMeasure<T, O = f64> {
    fn compute(&self, a: T, b: T) -> Result<O, SimilarityMeasureError>;
}

pub trait GraphInducer<T> {
    fn induce(&self, vals: &[T]) -> HashSet<T>;
}

pub struct IntersectionSimilarity {
    inducer: Box<dyn GraphInducer<TermId>>,
}

impl SimilarityMeasure<&Subject<TermId>> for IntersectionSimilarity {
    fn compute(
        &self,
        a: &Subject<TermId>,
        b: &Subject<TermId>,
    ) -> Result<f64, SimilarityMeasureError> {
        let ai = self.inducer.induce(&a.anns);
        let bi = self.inducer.induce(&b.anns);

        Ok(ai.intersection(&bi).count() as f64)
    }
}

pub struct JaccardSimilarity {
    inducer: Box<dyn GraphInducer<TermId>>,
}

impl SimilarityMeasure<&Subject<TermId>> for JaccardSimilarity {
    fn compute(
        &self,
        a: &Subject<TermId>,
        b: &Subject<TermId>,
    ) -> Result<f64, SimilarityMeasureError> {
        let ai = self.inducer.induce(&a.anns);
        let bi = self.inducer.induce(&b.anns);

        let i = ai.intersection(&bi).count();
        let u = ai.union(&bi).count();
        Ok((i as f64) / (u as f64))
    }
}

#[cfg(test)]
mod test_sim_measures {
    use std::collections::HashSet;

    use ontolius::TermId;

    use crate::similarity::{
        GraphInducer, IntersectionSimilarity, JaccardSimilarity, SimilarityMeasure, Subject,
    };

    #[derive(Clone, Debug)]
    struct FakeInducer;

    impl<T> GraphInducer<T> for FakeInducer {
        fn induce(&self, _vals: &[T]) -> HashSet<T> {
            HashSet::new()
        }
    }

    #[test]
    fn test_them() {
        let inducer = FakeInducer;

        let a = Subject { anns: vec![] };
        let b = Subject { anns: vec![] };

        let sims: [Box<dyn SimilarityMeasure<&Subject<TermId>>>; 2] = [
            Box::new(IntersectionSimilarity {
                inducer: Box::new(inducer.clone()),
            }),
            Box::new(JaccardSimilarity {
                inducer: Box::new(inducer.clone()),
            }),
        ];

        for ele in sims {
            compute_similarity_dyn_dispatch(&*ele, &a, &b);
            compute_similarity_static_dispatch(&*ele, &a, &b);
            compute_similarity(&ele, &a, &b);
        }
    }

    fn compute_similarity<T>(
        // Not too bad as an API
        sm: &Box<dyn SimilarityMeasure<T>>,
        a: T,
        b: T,
    ) -> f64 {
        sm.compute(a, b).unwrap()
    }

    fn compute_similarity_dyn_dispatch<T>(
        // Not too bad as an API
        sm: &dyn SimilarityMeasure<T>,
        a: T,
        b: T,
    ) -> f64 {
        sm.compute(a, b).unwrap()
    }

    fn compute_similarity_static_dispatch<SM, T>(
        // Not too bad as an API
        sm: &SM,
        a: T,
        b: T,
    ) -> f64
    where
        SM: SimilarityMeasure<T> + ?Sized,
    {
        sm.compute(a, b).unwrap()
    }
}
