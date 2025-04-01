use std::ops::Add;

/// A `Fraction` represents the *n* of *m* frequency of a feature in one or more annotated items.
///
/// For instance, we can represent the number of times *n* a feature
/// such as [Polydactyly](https://hpo.jax.org/browse/term/HP:0010442) was present
/// in a cohort of *m* individuals.
///
/// The counts are accessible via the `numerator` and `denominator` methods.
///
/// The `numerator` must be less than or equal to `denominator`.
/// However, it is possible for both to equal to `0`.
///
/// `Fraction` is generic over the data type that represents the count.
/// To simplify the API, we use `u32` as default.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fraction<T = u32> {
    n: T,
    /// The `m` represents the total count of annotated items investigated
    /// for presence/absence of the feature.
    m: T,
}

impl<T> Fraction<T>
where
    T: Clone,
{
    /// Get the value of the numerator.
    pub fn n(&self) -> T {
        Clone::clone(&self.n)
    }

    /// Get the value of the denominator.
    pub fn m(&self) -> T {
        Clone::clone(&self.m)
    }
}

/// Convert a tuple with numerator and denominator into the `Fraction`.
///
/// ## Examples
///
/// Parsing of `1/10` should succeed:
///
/// ```
/// use phenotypes::Fraction;
///
/// let f = Fraction::try_from((1, 10)).expect("Should never fail for this input");
///
/// assert_eq!(f.n(), 1);
/// assert_eq!(f.m(), 10);
/// ```
///
/// but we get an error if the numerator is greater than the denominator:
///
/// ```
/// use phenotypes::Fraction;
///
/// let err = Fraction::try_from((5, 3)).unwrap_err();
/// assert_eq!(err, "Numerator must be less than or equal to denominator!");
/// ```
impl<T> TryFrom<(T, T)> for Fraction<T>
where
    T: PartialOrd,
{
    type Error = &'static str;

    fn try_from(value: (T, T)) -> Result<Self, Self::Error> {
        let (numerator, denominator) = value;
        if numerator <= denominator {
            Ok(Self {
                n: numerator,
                m: denominator,
            })
        } else {
            Err("Numerator must be less than or equal to denominator!")
        }
    }
}

/// Make a new `Fraction` by summing up *n* and *m* values.
///
/// ```
/// use phenotypes::Fraction;
///
/// let a = Fraction::try_from((1, 2)).unwrap();
/// let b = Fraction::try_from((3, 3)).unwrap();
///
/// let c = a + b;
///
/// assert_eq!(c.n(), 4);
/// assert_eq!(c.m(), 5);
/// ```
impl<T> Add<Self> for Fraction<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Fraction {
            n: self.n + rhs.n,
            m: self.m + rhs.m,
        }
    }
}
