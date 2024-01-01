# gito

A CLI to expand the ability of `git`.

# install

`cargo install gito`

# Commands

## get-upstream

> alias: gup

get the parent repo ssh url based on github relationship if there is and set it as `upstream` remote

```bash
gito gup --remote-name [name] # default is `upstream`
```

![gito gup](https://user-images.githubusercontent.com/49113249/231788513-3a51e36f-801f-405d-b0dd-763cef906297.gif)

## user

manage git user.

it's very useful when you have a few git accounts, like one is work account and one is github account.

there're some sub-commands inspired by [nrm](https://github.com/Pana/nrm), you can see the detail by running `gito user -h`.

Here we give some example

```bash
gito user add github HomyeeKing HomyeeKing@gmail.com
gito user ls
```

you will see an output like

```
+--------+------------+----------------------+
| alias  | name       | email                |
+--------+------------+----------------------+
| github | HomyeeKing | HomyeeKing@gmail.com |
+--------+------------+----------------------+
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
# git commit --amend --author 'HomyeeKing <HomyeeKing@gmail.com>' --no-edit
#git rebase --continue #
```

## init

git init with specific user info by alias

```bash
gito init github
```

## branch

some branch action

```bash
gito branch delete <branch_name> # delete branch both locally and remotely
```