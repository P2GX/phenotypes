use ontolius::Identified;

/// An `Observable` entity is either in a *present* or an *excluded* state
/// in the investigated item.
///
/// For instance, a phenotypic feature such as [Polydactyly](https://hpo.jax.org/browse/term/HP:0010442)
/// can either be present or excluded in the study subject.
pub trait Observable {
    /// Test if the feature was observed in one or more items.
    fn is_present(&self) -> bool;
    /// Test if the feature was not observed in any of the items.
    fn is_excluded(&self) -> bool {
        !self.is_present()
    }
}

impl<T> Observable for &'_ T
where
    T: Observable,
{
    fn is_present(&self) -> bool {
        (*self).is_present()
    }
}

impl<T> Observable for Box<T>
where
    T: Observable,
{
    fn is_present(&self) -> bool {
        (**self).is_present()
    }
}

/// Common functionalities for containers of the [`Observable`] features.
pub trait ObservableFeatures {
    /// The feature.
    type Feature: Identified + Observable;

    /// Get an iterator over features that were observed in the investigated item.
    fn present_features(&self) -> impl Iterator<Item = Self::Feature>;

    /// Get the number of observed features.
    fn present_feature_count(&self) -> usize {
        self.present_features().count()
    }

    /// Get an iterator over features whose presence was specifically excluded in the investigated item.
    fn excluded_features(&self) -> impl Iterator<Item = Self::Feature>;

    /// Get the number of features whose presence was specifically excluded.
    fn excluded_feature_count(&self) -> usize {
        self.excluded_features().count()
    }
}

impl<'a, T> ObservableFeatures for &'a [T]
where
    &'a T: Identified + Observable,
{
    type Feature = &'a T;

    fn present_features(&self) -> impl Iterator<Item = Self::Feature> {
        self.iter().filter(|&t| t.is_present())
    }

    fn excluded_features(&self) -> impl Iterator<Item = Self::Feature> {
        self.iter().filter(|&t| t.is_excluded())
    }
}
