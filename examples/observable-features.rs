//! Example usage of the observable features.
#![cfg(test)]

use ontolius::TermId;
use phenotypes::{
    Fraction, ObservableFeatures,
    feature::{AggregatedFeature, IndividualFeature},
};

/// We create an individual consisting of 2 observed and 1 excluded HPO term
/// and then test that we can get the observed/excluded term counts.
#[test]
fn individual() {
    // An individual with excluded seizures and observed arachnodactyly and hypertension.
    let individual: Vec<_> = [
        ("HP:0001250", false),
        ("HP:0001166", true),
        ("HP:0000822", true),
    ]
    .into_iter()
    .map(|(curie, is_present)| {
        let term_id: TermId = curie.parse().expect("CURIE is valid");
        IndividualFeature::from((term_id, is_present))
    })
    .collect();

    let pfc = (&individual[..]).present_feature_count();
    let efc = (&individual[..]).excluded_feature_count();

    // We expect two present features ...
    assert_eq!(pfc, 2);
    // ... and one excluded.
    assert_eq!(efc, 1);
}

#[test]
fn aggregated() {
    let aggregated: Vec<_> = [
        ("HP:0001250", (4, 5)),
        ("HP:0001166", (0, 5)),
        ("HP:0000822", (0, 8)),
    ]
    .into_iter()
    .map(|(curie, counts)| {
        let term_id: TermId = curie.parse().expect("CURIE is valid");
        let f = Fraction::try_from(counts).expect("OK");
        AggregatedFeature::from((term_id, f))
    })
    .collect();

    let pfc = (&aggregated[..]).present_feature_count();
    let efc = (&aggregated[..]).excluded_feature_count();

    // We expect one present feature ...
    assert_eq!(pfc, 1);
    // ... and two excluded.
    assert_eq!(efc, 2);
}
