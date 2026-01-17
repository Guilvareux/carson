/// Rust-based test harness for Prolog reasoning engine
///
/// This module provides Rust tests that:
/// 1. Initialize the Prolog engine
/// 2. Load ontologies and scenarios
/// 3. Execute Prolog test suites
/// 4. Report results to Rust test framework

use crate::prolog_engine::PrologEngine;
use swipl::context::{Context, ActivatedEngine};
use swipl::init::{activate_main, initialize_swipl};

/// Helper to initialize Prolog context for tests
fn setup_prolog_context() -> Context<'static, ActivatedEngine<'static>> {
    let activation = initialize_swipl().unwrap_or_else(|| activate_main());
    activation.into()
}

#[test]
fn test_core_modules_load() {
    let context = setup_prolog_context();

    PrologEngine::initialize(&context)
        .expect("Failed to initialize Prolog");

    PrologEngine::load_core_modules(&context)
        .expect("Failed to load core Prolog modules");
}

#[test]
fn test_ontologies_load() {
    let context = setup_prolog_context();

    PrologEngine::initialize(&context)
        .expect("Failed to initialize Prolog");

    PrologEngine::load_core_modules(&context)
        .expect("Failed to load core modules");

    PrologEngine::load_ontologies(&context, "rdf/ontologies")
        .expect("Failed to load ontologies");
}

#[test]
fn test_simple_scenario_loads() {
    let context = setup_prolog_context();

    PrologEngine::initialize(&context)
        .expect("Failed to initialize Prolog");

    PrologEngine::load_core_modules(&context)
        .expect("Failed to load core modules");

    PrologEngine::load_ontologies(&context, "rdf/ontologies")
        .expect("Failed to load ontologies");

    PrologEngine::load_scenario(&context, "rdf/scenarios/network/simple_network.ttl")
        .expect("Failed to load simple network scenario");
}

#[test]
fn test_oz_scenario_loads() {
    let context = setup_prolog_context();

    PrologEngine::initialize(&context)
        .expect("Failed to initialize Prolog");

    PrologEngine::load_core_modules(&context)
        .expect("Failed to load core modules");

    PrologEngine::load_ontologies(&context, "rdf/ontologies")
        .expect("Failed to load ontologies");

    PrologEngine::load_scenario(&context, "rdf/scenarios/services/oz.ttl")
        .expect("Failed to load oz service scenario");
}

#[test]
fn test_framework_suite() {
    let context = setup_prolog_context();

    PrologEngine::initialize(&context)
        .expect("Failed to initialize Prolog");

    // Load everything needed
    PrologEngine::load_core_modules(&context)
        .expect("Failed to load core modules");

    // Load simple scenario for simple_scenario tests
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
        .unwrap_or_else(|_| env!("CARGO_MANIFEST_DIR").to_string());
    let simple_scenario = format!("{}/prolog/scenarios/simple_network.pl", manifest_dir);
    let term = context.new_term_ref();
    term.unify(&simple_scenario).unwrap();
    context.call_once(swipl::prelude::pred!(consult/1), [&term])
        .expect("Failed to load simple scenario predicates");

    // Run framework self-tests
    PrologEngine::run_test_suite(&context, "framework")
        .expect("Framework test suite failed");
}

#[test]
fn test_simple_scenario_suite() {
    let context = setup_prolog_context();

    PrologEngine::initialize(&context)
        .expect("Failed to initialize Prolog");

    PrologEngine::load_core_modules(&context)
        .expect("Failed to load core modules");

    // Load simple scenario predicates
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
        .unwrap_or_else(|_| env!("CARGO_MANIFEST_DIR").to_string());
    let simple_scenario = format!("{}/prolog/scenarios/simple_network.pl", manifest_dir);
    let term = context.new_term_ref();
    term.unify(&simple_scenario).unwrap();
    context.call_once(swipl::prelude::pred!(consult/1), [&term])
        .expect("Failed to load simple scenario predicates");

    PrologEngine::run_test_suite(&context, "simple_scenario")
        .expect("Simple scenario test suite failed");
}

#[test]
fn test_ontology_suite() {
    let context = setup_prolog_context();

    PrologEngine::initialize(&context)
        .expect("Failed to initialize Prolog");

    PrologEngine::load_core_modules(&context)
        .expect("Failed to load core modules");

    PrologEngine::load_ontologies(&context, "rdf/ontologies")
        .expect("Failed to load ontologies");

    PrologEngine::load_scenario(&context, "rdf/scenarios/services/oz.ttl")
        .expect("Failed to load oz scenario");

    PrologEngine::run_test_suite(&context, "ontology")
        .expect("Ontology test suite failed");
}

#[test]
fn test_intent_driven_suite() {
    let context = setup_prolog_context();

    PrologEngine::initialize(&context)
        .expect("Failed to initialize Prolog");

    PrologEngine::load_core_modules(&context)
        .expect("Failed to load core modules");

    PrologEngine::load_ontologies(&context, "rdf/ontologies")
        .expect("Failed to load ontologies");

    PrologEngine::load_scenarios(&context, &[
        "rdf/scenarios/services/oz.ttl",
        "rdf/scenarios/network/simple_network.ttl",
    ]).expect("Failed to load scenarios");

    PrologEngine::run_test_suite(&context, "intent_driven")
        .expect("Intent-driven test suite failed");
}

/// Integration test: Full test run (mimics old `just test`)
#[test]
#[ignore] // Run with: cargo test -- --ignored
fn test_all_suites() {
    let context = setup_prolog_context();

    PrologEngine::initialize(&context)
        .expect("Failed to initialize Prolog");

    // Setup
    PrologEngine::load_core_modules(&context)
        .expect("Failed to load core modules");

    PrologEngine::load_ontologies(&context, "rdf/ontologies")
        .expect("Failed to load ontologies");

    PrologEngine::load_scenarios(&context, &[
        "rdf/scenarios/services/oz.ttl",
        "rdf/scenarios/network/simple_network.ttl",
        "rdf/scenarios/network/bt-uk.ttl",
    ]).expect("Failed to load scenarios");

    // Load simple scenario predicates for simple_scenario tests
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
        .unwrap_or_else(|_| env!("CARGO_MANIFEST_DIR").to_string());
    let simple_scenario = format!("{}/prolog/scenarios/simple_network.pl", manifest_dir);
    let term = context.new_term_ref();
    term.unify(&simple_scenario).unwrap();
    context.call_once(swipl::prelude::pred!(consult/1), [&term])
        .expect("Failed to load simple scenario predicates");

    // Run all tests
    PrologEngine::run_all_tests(&context)
        .expect("Test suite failed");
}
