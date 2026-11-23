use jellyfish::dynamics::Attractor;
use jellyfish::scene::config::AttractorConfig;

#[test]
fn lorenz_parameters() {
    let mut config = AttractorConfig::lorenz();

    assert_eq!(config.system.parameter_specs().len(), 3);
    assert_eq!(config.system.parameter_value(1), Some(28.0));

    assert!(config.system.set_parameter(1, 35.0));
    assert_eq!(config.system.parameter_value(1), Some(35.0));

    assert!(matches!(config.system, Attractor::Lorenz(_)));
}

#[test]
fn rossler_parameters() {
    let mut config = AttractorConfig::rossler();

    assert_eq!(config.system.parameter_specs().len(), 3);
    assert_eq!(config.system.parameter_value(2), Some(5.7));

    assert!(config.system.set_parameter(2, 8.0));
    assert_eq!(config.system.parameter_value(2), Some(8.0));
}

#[test]
fn invalid_parameters_are_rejected() {
    let mut config = AttractorConfig::rossler();

    assert!(!config.system.set_parameter(99, 1.0));
    assert!(!config.system.set_parameter(0, f64::NAN));
    assert!(!config.system.set_parameter(0, f64::INFINITY));
    assert!(!config.system.set_parameter(0, -10.0));

    assert_eq!(config.system.parameter_value(0), Some(0.2));
    assert_eq!(config.system.parameter_value(99), None);
}
