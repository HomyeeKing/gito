# gito

一个扩展 `git` 能力的命令行工具。

# 安装

`cargo install gito`

# 命令

## get-upstream

> 别名: gup

根据 GitHub 关系获取上游仓库的 SSH URL，并将其设置为 `upstream` 远程。

更多详情，请运行 `gito get-upstream -h`。

![gito gup](https://user-images.githubusercontent.com/49113249/231788513-3a51e36f-801f-405d-b0dd-763cef906297.gif)

## user

管理 Git 用户。

当您有多个 Git 账户时（例如，工作账户和 GitHub 账户），这非常有用。

更多详情，请运行 `gito user -h`。

## amend

通过别名修改提交的作者和邮箱。当您忘记切换账户时非常有用。

更多详情，请运行 `gito amend -h`。

## init

使用特定用户别名初始化 Git 仓库。

更多详情，请运行 `gito init -h`。

## open

打开与当前 Git 仓库相关的网站。此命令允许您快速在浏览器中打开预定义或自定义的 URL。

`base_url` 现在可以包含占位符：
- `<group>`: 替换为 `username/repo` 的第一部分（例如，`username` 在 `username/repo` 中）。
- `<name>`: 替换为 `username/repo` 的第二部分（例如，`repo` 在 `username/repo` 中）。
- `<branch>`: 替换为当前 Git 仓库的分支名称。

如果未使用占位符，命令将自动将当前仓库的 `username/repo` 路径附加到 `base_url`。

更多详情，请运行 `gito open -h`。

## branch

执行各种分支操作。

`<branch>` 关键词是一个占位符，表示 Git 分支的名称。它用于需要指定特定分支的命令中，例如删除分支。

更多详情，请运行 `gito branch -h`。
