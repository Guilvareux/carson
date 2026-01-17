/// Prolog Engine Management
/// This module manages the lifecycle of the SWI-Prolog engine and provides
/// high-level operations for loading ontologies, scenarios, and running queries.

use anyhow::{Context as AnyhowContext, Result};
use std::path::PathBuf;
use swipl::context::{Context, QueryableContextType};
use swipl::prelude::*;

/// Prolog engine wrapper that manages loading and querying
pub struct PrologEngine;

impl PrologEngine {
    /// Initialize Prolog environment (libraries, prefixes, flags)
    pub fn initialize(context: &Context<'_, impl QueryableContextType>) -> Result<()> {
        Self::load_prolog_file(context, "prolog/init.pl")
            .context("Failed to initialize Prolog environment")?;
        Ok(())
    }

    /// Load core Prolog modules (reasoner, ontology utilities, etc.)
    pub fn load_core_modules(context: &Context<'_, impl QueryableContextType>) -> Result<()> {
        Self::load_prolog_file(context, "prolog/util.pro")
            .context("Failed to load util module")?;

        Self::load_prolog_file(context, "prolog/ontology.pl")
            .context("Failed to load ontology module")?;

        Self::load_prolog_file(context, "prolog/reasoner.pl")
            .context("Failed to load reasoner module")?;

        // Load test framework (for assertions)
        Self::load_prolog_file(context, "prolog/tests.pl")
            .context("Failed to load test framework")?;

        Ok(())
    }

    /// Load all ontologies from a directory
    pub fn load_ontologies(
        context: &Context<'_, impl QueryableContextType>,
        ontology_dir: &str,
    ) -> Result<()> {
        eprintln!("Loading ontologies from: {}", ontology_dir);

        // Call the Prolog predicate load_all_ontologies/1
        let term = context.new_term_ref();
        term.unify(ontology_dir)?;

        context.call_once(pred!(load_all_ontologies/1), [&term])
            .context("Failed to load ontologies")?;

        Ok(())
    }

    /// Load a single scenario file
    pub fn load_scenario(
        context: &Context<'_, impl QueryableContextType>,
        scenario_path: &str,
    ) -> Result<()> {
        eprintln!("Loading scenario: {}", scenario_path);

        let term = context.new_term_ref();
        term.unify(scenario_path)?;

        context.call_once(pred!(load_scenario/1), [&term])
            .context(format!("Failed to load scenario: {}", scenario_path))?;

        Ok(())
    }

    /// Load multiple scenarios
    pub fn load_scenarios<P: AsRef<str>>(
        context: &Context<'_, impl QueryableContextType>,
        scenarios: &[P],
    ) -> Result<()> {
        for scenario in scenarios {
            Self::load_scenario(context, scenario.as_ref())?;
        }
        Ok(())
    }

    /// Run all tests in the test framework
    pub fn run_all_tests(context: &Context<'_, impl QueryableContextType>) -> Result<()> {
        eprintln!("\n=== Running All Netwits Tests ===\n");

        context.call_once(pred!(run_all_tests/0), [])
            .context("Test suite failed")?;

        Ok(())
    }

    /// Run a specific test suite
    pub fn run_test_suite(
        context: &Context<'_, impl QueryableContextType>,
        suite_name: &str,
    ) -> Result<()> {
        eprintln!("\n=== Running Test Suite: {} ===\n", suite_name);

        let term = context.new_term_ref();
        term.unify(suite_name)?;

        context.call_once(pred!(run_test_suite/1), [&term])
            .context(format!("Test suite '{}' failed", suite_name))?;

        Ok(())
    }

    /// Load a Prolog file
    fn load_prolog_file(
        context: &Context<'_, impl QueryableContextType>,
        relative_path: &str,
    ) -> Result<()> {
        let full_path = Self::prolog_file_path(relative_path);

        if !full_path.exists() {
            anyhow::bail!("Prolog file does not exist: {}", full_path.display());
        }

        eprintln!("Loading: {}", full_path.display());

        let term = context.new_term_ref();
        term.unify(full_path.to_string_lossy().as_ref())?;
        context.call_once(pred!(consult/1), [&term])?;

        Ok(())
    }

    /// Resolve relative path to absolute path
    fn prolog_file_path(relative_path: &str) -> PathBuf {
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
            .unwrap_or_else(|_| env!("CARGO_MANIFEST_DIR").to_string());
        PathBuf::from(manifest_dir).join(relative_path)
    }
}
