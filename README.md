<p align="center"><img src="./public/gito.svg" alt="gito logo" width="100" style="border-radius:50%" /></p>

<h1 align="center">Gito <small>/dʒɪtəʊ/ or /gɪtəʊ/</small></h1>

<p align="center">
A CLI to expand the ability of `git`.
</p>

[中文介绍请移步](README-zh.md)

# install

`cargo install gito`

# Commands

## get-upstream

> alias: gup

get the parent repo ssh url based on github relationship if there is and set it as `upstream` remote

```bash
gito gup --remote-name [name] # default is `upstream`
gito gup # equals to `gito gup --remote-name upstream`
```

![gito gup](https://user-images.githubusercontent.com/49113249/231788513-3a51e36f-801f-405d-b0dd-763cef906297.gif)

## user

manage git user.

it's very useful when you have a few git accounts, like one is work account and one is github account.

there're some sub-commands inspired by [nrm](https://github.com/Pana/nrm), you can see the detail by running `gito user -h`.

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

Open websites related to the current git repository. This command allows you to quickly open predefined or custom URLs in your browser.

Assume you have this config on your device:
```
+----------+-------------------------------------------------+
| alias    | base_url                                        |
+----------+-------------------------------------------------+
| deepwiki | https://deepwiki.com/                           |
+----------+-------------------------------------------------+
| github   | https://github.com/|
+----------+-------------------------------------------------+
```
And your working directory is `git@github.com:HomyeeKing/gito.git` project. 
```bash
gito open github # will open https://github.com/HomyeeKing/gito
```

And the `base_url` can now include placeholders:
- `<group>`: Replaced by the first part of `group/name`
- `<name>`: Replaced by the second part of `group/name`
- `<branch>`: Replaced by the current Git repository's branch name.

You can change the github base_url as `https://github.com/<group>/<name>/tree/<branch>`
```
+----------+-------------------------------------------------+
| alias    | base_url                                        |
+----------+-------------------------------------------------+
| deepwiki | https://deepwiki.com/                           |
+----------+-------------------------------------------------+
| github   | https://github.com/<group>/<name>/tree/<branch>|
+----------+-------------------------------------------------+
```

This time the browser will open if you are working on branch `feat/foo` `https://github.com/HomyeeKing/gito/tree/feat/foo`.

So You can assemble any base_url with it~

## branch

some branch action

```bash
gito branch delete <branch_name> # delete branch both locally and remotely
```
