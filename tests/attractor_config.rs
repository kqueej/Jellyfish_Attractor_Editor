use jellyfish::dynamics::Attractor;
use jellyfish::scene::config::AttractorConfig;

#[test]
fn lorenz_config_generates_vertices() {
    let config = AttractorConfig::lorenz();

    let vertices = config.vertices();

    assert_eq!(vertices.len(), config.count + 1);
}

#[test]
fn rossler_config_generates_vertices() {
    let config = AttractorConfig::rossler();

    let vertices = config.vertices();

    assert_eq!(vertices.len(), config.count + 1);
}

#[test]
fn changing_parameter_changes_trajectory() {
    let mut config = AttractorConfig::lorenz();

    config.count = 300;
    config.warmup = 50;

    let original = config.vertices();

    if let Attractor::Lorenz(system) = &mut config.system {
        system.rho = 35.0;
    }

    let modified = config.vertices();

    assert_eq!(original.len(), modified.len());

    assert!(
        original
            .iter()
            .zip(modified.iter())
            .any(|(a, b)| a.position != b.position)
    );
}
