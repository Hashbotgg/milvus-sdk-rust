// Licensed to the LF AI & Data foundation under one
// or more contributor license agreements. See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership. The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License. You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::{
    error::Error,
    proto::common::{ErrorCode, Status},
};

pub fn status_to_result(status: &Option<Status>) -> Result<(), Error> {
    let status = status
        .clone()
        .ok_or(Error::Unexpected("no status".to_owned()))?;

    match ErrorCode::try_from(status.error_code) {
        Ok(i) => match i {
            ErrorCode::Success => Ok(()),
            _ => Err(Error::from(status)),
        },
        Err(_) => Err(Error::Unexpected(format!(
            "unknown error code {}",
            status.error_code
        ))),
    }
}
