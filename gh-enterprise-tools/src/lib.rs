/*
 * gh-enterprise-tools
 * Copyright (C) 2023 Mike Stemle
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::env;

pub fn gh_auth() -> octocrab::Octocrab {
  let token = match env::var("GITHUB_TOKEN") {
    Ok(val) => val,
    Err(err) => panic!("Unable to get a usable GitHub token: {:#?}", err),
  };
  match octocrab::OctocrabBuilder::default()
    .user_access_token(token)
    .build()
  {
    Ok(gh) => gh,
    Err(err) => panic!("Error creating GitHub client: {:#?}", err),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[tokio::test]
  async fn test_gh_auth() {
    let result = gh_auth();
    assert!(result.current().user().await.is_ok());
  }
}
