# gito

A CLI to expand the ability of `git`.

# install

`cargo install gito`

# Commands

## get-upstream

> alias: gup

Get the parent repo ssh url based on github relationship if there is and set it as `upstream` remote.

For more details, run `gito get-upstream -h`.

![gito gup](https://user-images.githubusercontent.com/49113249/231788513-3a51e36f-801f-405d-b0dd-763cef906297.gif)

## user

Manage git user accounts. It's very useful when you have multiple git accounts (e.g., work and personal).

For more details, run `gito user -h`.

## amend

Amend the commit's author and email by alias. Useful when you forget to change accounts.


Here we give some example

```bash
gito user add github foo foo@example.com
gito user ls
```

you will see an output like

```
+--------+-------+-----------------+
| alias  | name  | email           |
+--------+-------+-----------------+
| github | foo   | foo@example.com |
+--------+-------+-----------------+
```

Similiarly, you can `delete` and `use` specific account by `alias`

## amend

as we may have different git account, sometimes we may forget to change account, so we have to run `git rebase -i <commit>` to amend it.

so here based on the `gito user`, you can amend by `alias`, the workflow like:

```bash
git rebase -i <commit> # choose commit need to be edit

# notice that `github` is the alias we create before
gito ammend github
# equivalent to these two commands
# git commit --amend --author 'foo <foo@example.com>' --no-edit
#git rebase --continue #
```

## init

git init with specific user info by alias

```bash
gito init github
```

## open

Open websites related to the current git repository. This command allows you to quickly open predefined or custom URLs in your browser, automatically appending the current repository's `username/repo` path.

You can define custom websites using `gito open add`. There is also a default `deepwiki` configuration.

### Subcommands

- `gito open ls`: List all registered websites (both user-defined and default).
- `gito open add <alias> <base_url>`: Add a new website alias with its base URL.
- `gito open del <alias>`: Delete a user-defined website alias.

### Usage Examples

Open a website using an alias:

```bash
# Opens https://deepwiki.com/foo/bar (if current repo is foo/bar)
gito open deepwiki 
```

Add a custom website:

```bash
gito open add myjira https://jira.mycompany.com/browse/
```

Then open it:

```bash
# Opens https://jira.mycompany.com/browse/foo/bar
gito open myjira
```

## branch

some branch action

```bash
gito branch delete <branch_name> # delete branch both locally and remotely
```
