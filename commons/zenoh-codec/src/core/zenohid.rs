use std::convert::TryFrom;

//
// Copyright (c) 2022 ZettaScale Technology
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at
// http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
// which is available at https://www.apache.org/licenses/LICENSE-2.0.
//
// SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
//
// Contributors:
//   ZettaScale Zenoh Team, <zenoh@zettascale.tech>
//
use crate::*;
use zenoh_buffers::{
    reader::{DidntRead, Reader},
    writer::{DidntWrite, Writer},
};
use zenoh_protocol::core::ZenohId;

impl<W> WCodec<&mut W, &ZenohId> for Zenoh060
where
    W: Writer,
{
    type Output = Result<(), DidntWrite>;

    fn write(self, writer: &mut W, x: &ZenohId) -> Self::Output {
        zcwrite!(self, writer, x.as_slice())
    }
}

impl<R> RCodec<&mut R, ZenohId> for Zenoh060
where
    R: Reader,
{
    type Error = DidntRead;

    fn read(self, reader: &mut R) -> Result<ZenohId, Self::Error> {
        let size: usize = self.read(&mut *reader)?;
        if size > ZenohId::MAX_SIZE {
            return Err(DidntRead);
        }

        let mut id = [0; ZenohId::MAX_SIZE];
        reader.read_exact(&mut id[..size])?;
        ZenohId::try_from(&id[..size]).map_err(|_| DidntRead)
    }
}
