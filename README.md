# The Unofficial GitHub Enterprise Toolbox

## Description

Managing GitHub at the enterprise level can be tricky, and can there are a lot of manual processes. Some of GitHub's enterprise features—such as Enterprise Managed Users (EMUs)—can simplify many of these tasks, but not all of them.

This suite of tools aims to provide you with the building blocks from which you may construct your own automated processes.

## Installation

Check out the code, and then run `cargo install` in the root directory. You can also run it in the root directory with `cargo run`.

## Usage

Instructions on how to use your project.

### Parameter conventions

Across the programs in the suite, the following CLI options are maintained consistently.

| Short | Long         | Description                                                                                                              |
| ----- | ------------ | ------------------------------------------------------------------------------------------------------------------------ |
| `-d`  | `--debug`    | Enable debugging                                                                                                         |
| `-e`  | `--existing` | If you're using a program which replaces one value for another, this parameter is where you supply the value to replace. |
| `-o`  | `--org`      | A GitHub organization slug.                                                                                              |
| `-q`  | `--quiet`    | Suppress all output except error messages.                                                                               |
| `-r`  | `--repo`     | A repository name                                                                                                        |
| `-R`  | `--role`     | A role in GitHub, either "member" or "owner"                                                                             |
| `-t`  | `--team`     | The slug for a GitHub Team. This is not the name, but the slug.                                                          |
| `-u`  | `--user`     | A GitHub user                                                                                                            |
| `-v`  | `--version`  | Print version information                                                                                                |
| `-V`  | `--verbose`  | Print verbose output                                                                                                     |

**Note:** For the `--existing` parameter, this can be a team, user, org, etc., whatever value you are _replacing_ with another. For example,
if I wanted to swap one team on a repository with another, I would use the command:

```sh
gh-repo-team-swap \
  --existing team-to-remove \
  --team team-to-add \
  --repo repo-name
```

**Another note:** For all of these arguments, you want to use a _slug_ instead of a _name_. If your team's name is "Quality Engineers", the slug is likely `quality-engineers` in GitHub. If you have any doubts, check the URL when you're viewing the entity. This applies to `--org` and `--team`.

## The Suite

This project contains a suite of tools, and while each tool does one thing, they are intended to be composed with scripts to automate your processes.

| Done? | Package Name            | Description                                                                                 |
| ----- | ----------------------- | ------------------------------------------------------------------------------------------- |
| :x:   | `gh-add-team`           | Add a team to an organization.                                                              |
| :x:   | `gh-enterprise-tools`   | This is a set of functions common to the program, not a program itself.                     |
| :x:   | `gh-invite-user`        | Invite a user to the org.                                                                   |
| :x:   | `gh-ls-teams`           | List all of the teams for the org.                                                          |
| :x:   | `gh-ls-users`           | List all of the users for the org.                                                          |
| :x:   | `gh-remove-team`        | Remove a team from the org.                                                                 |
| :x:   | `gh-remove-user`        | Remove a user from the org.                                                                 |
| :hourglass:   | `gh-repo-team-swap`     | Remove one team from a repository, and replace it with another one using the role provided. |
| :x:   | `gh-team-invite-member` | Invite a member (user) to a team.                                                           |
| :x:   | `gh-team-remove-member` | Remove a member (user) from a team.                                                         |

## Contributing

Please see `CONTRIBUTING.md` for information on how to contribute to this project.

## License

This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
