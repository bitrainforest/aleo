// Copyright (C) 2019-2021 Aleo Systems Inc.
// This file is part of the Aleo library.

// The Aleo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Aleo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Aleo library. If not, see <https://www.gnu.org/licenses/>.


use crate::{Network};
use snarkvm::{file::Manifest, package::Package};
use snarkvm::prelude::{
    Transaction,
    Value,
    VM,
};

use anyhow::{ensure, bail, Result};
use clap::Parser;
use ureq;

/// Executes an Aleo program function.
#[derive(Debug, Parser)]
pub struct Deploy {
    /// The record inputs.
    #[clap(parse(try_from_str))]
    record: Value<Network>,
    /// Uses the specified endpoint.
    #[clap(long)]
    endpoint: Option<String>,
}

impl Deploy {
    /// Deploy an Aleo program.
    pub fn parse(self) -> Result<String> {
        // Derive the program directory path.
        let directory = std::env::current_dir()?;

        // Ensure the directory path exists.
        ensure!(
            directory.exists(),
            "The program directory does not exist: {}",
            directory.display()
        );
        // Ensure the manifest file exists.
        ensure!(
            Manifest::<Network>::exists_at(&directory),
            "Please deploy in an Aleo program directory (missing '{}' at '{}')",
            Manifest::<Network>::file_name(),
            directory.display()
        );

        // Open the manifest file.
        let manifest = Manifest::open(&directory)?;

        // Retrieve the private key.
        let private_key = manifest.development_private_key();

        // Load the package.
        let package = Package::open(&directory)?;
        // Load the program.
        let program = package.program();

        // Retrieve the record.
        let record = match self.record {
            Value::Record(record) => record,
            // Ensure the input is a record.
            Value::Plaintext(..) => bail!("Expected a record input, found a plaintext input"),
        };

        let additional_fee= 1;
        ensure!(
             ***record.gates() >= additional_fee,
            "The additional fee exceeds the record balance."
        );

        let vm = VM::<Network>::new()?;

        // Deploy.
        let transaction = Transaction::deploy(
            &vm,
            &private_key,
            program,
            (record, additional_fee),
            &mut rand::thread_rng(),
        )?;

        // Verify.
        assert!(vm.verify(&transaction));

        println!("sending transaction broadcast...");

        match self.endpoint {
            Some(ref endpoint) => {
                Ok(ureq::post(endpoint).send_json(transaction)?.into_string()?)
            }
            None => {
                Ok(ureq::post("http://127.0.0.1:4180/testnet3/transaction/broadcast").send_json(transaction)?.into_string()?)
            }
        }
    }
}
