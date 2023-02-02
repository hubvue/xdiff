use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};

use crate::ExtraArgs;

/// Diff two http requests and compare the difference of the responses
#[derive(Parser, Debug, Clone)]
#[clap(version, author, about, long_about = None)]
pub struct Args {
    #[clap(subcommand)]
    pub action: Action,
}

#[derive(Subcommand, Debug, Clone)]
#[non_exhaustive]
pub enum Action {
    Run(RunArgs),
}

#[derive(Parser, Debug, Clone)]
pub struct RunArgs {
    /// Profile name
    #[clap(short, long, value_parser)]
    pub profile: String,

    #[clap(short, long, value_parser = parse_key_val, number_of_values = 1)]
    pub extra_params: Vec<KeyVal>,

    /// Configuration to use.
    #[clap(short, long, value_parser)]
    pub config: Option<String>,
}

#[derive(Debug, Clone)]
pub struct KeyVal {
    pub key_type: KeyValType,
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone)]
pub enum KeyValType {
    Query,
    Header,
    Body,
}

fn retrieve<'a>(v: Option<&'a str>, s: &str) -> Result<&'a str> {
    Ok(v.ok_or_else(|| anyhow!("Invalid key value pair: {}", s))?
        .trim())
}

fn parse_key_val(s: &str) -> Result<KeyVal> {
    let mut parts = s.splitn(2, "=");

    let key = retrieve(parts.next(), s)?;
    let value = retrieve(parts.next(), s)?;

    let (key_type, key) = match key.chars().next() {
        Some('%') => (KeyValType::Header, &key[1..]),
        Some('@') => (KeyValType::Body, &key[1..]),
        Some(v) if v.is_ascii_alphabetic() => (KeyValType::Query, key),
        _ => return Err(anyhow!("Invalid key value pair")),
    };
    Ok(KeyVal {
        key_type,
        key: key.to_string(),
        value: value.to_string(),
    })
}

impl From<Vec<KeyVal>> for ExtraArgs {
    fn from(args: Vec<KeyVal>) -> Self {
        let mut headers = vec![];
        let mut query = vec![];
        let mut body = vec![];

        for arg in args {
            match arg.key_type {
                KeyValType::Header => headers.push((arg.key, arg.value)),
                KeyValType::Query => query.push((arg.key, arg.value)),
                KeyValType::Body => body.push((arg.key, arg.value)),
            }
        }
        Self {
            headers,
            query,
            body,
        }
    }
}
