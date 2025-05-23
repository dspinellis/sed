// Support for in-place editing
//
// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Diomidis Spinellis
//
// This file is part of the uutils sed package.
// It is licensed under the MIT License.
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use crate::command::ProcessingContext;
use crate::fast_io::OutputBuffer;
use std::io::stdout;
use std::path::Path;
use uucore::error::UResult;

/// Context for in-place editing
pub struct InPlace {
    pub output: OutputBuffer,
    pub processing_context: ProcessingContext,
}

impl InPlace {
    /// Create a new `ProcessingContext` taking ownership of processing_context
    pub fn new(processing_context: ProcessingContext) -> UResult<Self> {
        let output = OutputBuffer::new(Box::new(stdout()));

        Ok(InPlace {
            output,
            processing_context,
        })
    }

    /// Return an OutputBuffer for outputting the edits to the specified file.
    pub fn begin(&mut self, _file_name: &Path) -> UResult<&mut OutputBuffer> {
        // TODO: Adjust output for in-place editing, if needed.
        Ok(&mut self.output)
    }

    /// Finish in-place editing.
    pub fn end(&mut self) -> UResult<()> {
        self.output.flush()?;
        // TODO: Rename and delete output file, if needed.
        Ok(())
    }
}
