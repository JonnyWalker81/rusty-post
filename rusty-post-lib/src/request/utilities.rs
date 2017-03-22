/*
 * Copyright © 2002-2017 Bluebeam Software, Inc. All Rights Reserved.
 * Creator: Jonathan Rothberg
 */
use std::io::{Write, Error, Result};
use std::fmt::Arguments;

pub struct EmptyWriter {}

impl EmptyWriter {
    pub fn new() -> EmptyWriter {
        EmptyWriter{}
    }
}

impl Write for EmptyWriter {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        return Ok(0);
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }

    fn write_all(&mut self, buf: &[u8]) -> Result<()> {
        Ok(())
    }

    fn write_fmt(&mut self, fmt: Arguments) -> Result<()> {
        Ok(())
    }

    fn by_ref(&mut self) -> &mut Self {
        self
    }
}
