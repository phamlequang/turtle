# Turtle - an interactive shell for micro-services development

[![pipeline](https://gitlab.com/phamlequang/turtle/badges/master/pipeline.svg)](https://gitlab.com/phamlequang/turtle/commits/master) [![license](https://img.shields.io/badge/license-MIT-green.svg)](https://gitlab.com/phamlequang/turtle/blob/master/LICENSE)

Turtle is built with the purpose of helping developers speed up their development process, especially in projects that use micro-services architecture.

One goal of turtle is to provide a set of simple commands to setup, run and manage all services on local machine using docker-compose. This will be useful for writing and running integration or system tests that often need some parts or even the whole system to be up and running. It will also help front-end developers when they develop UI that needs to call APIs on multiple services.

Another goal of turtle is to still allow user to run any normal shell commands when they're inside its shell. It just helps speed up by providing more shortcuts. As of now, some shortcut commands to work with git and docker are supported, kubernetes are comming up. In the future, user will be able to easily define their own shortcut commands.

## Setup

## Commands

The following list contains supported shortcut commands of current turtle version. All example commands are using the [sample config file](https://gitlab.com/phamlequang/turtle/blob/master/etc/config.toml) in `etc` folder.

Command | Description | Example
--------|-------------|--------
`quit`|Exit turtle shell|`➜ quit`
`exit`|Exit turtle shell|`➜ exit`
`cd {path}`|Change current working directory|`➜ cd ~/projects/flowers/camellia/`
`goto {repository}`|Change current working directory to repository directory|`➜ goto flowers`
`goto {service}`|Change current working directory to service directory|`➜ goto lotus`
`clone`|Clone all repositories|`➜ clone`
`clone [repository1] [repository2] ...`|Clone a list of space-separated repositories|`➜ clone flowers trees`
`pull`|Git pull latest code on current branch of current working directory's repository|`➜ pull`
`pull [repository1] [repository2] ...`|Git pull latest code on current branch of the listed space-separated repositories]|`➜ pull flowers trees`
`pull [service1] [service2] ...`|Git pull latest code on current branch of the repositories of the listed space-separated services]|`➜ pull camellia lotus`
`push`|Git push latest code on current branch of current working directory's repository|`➜ push`
`push [repository1] [repository2] ...`|Git push latest code on current branch of the listed space-separated repositories]|`➜ push flowers trees`
`push [service1] [service2] ...`|Git push latest code on current branch of the repositories of the listed space-separated services]|`➜ push camellia lotus`
`dkmc {command}`|Run any docker-machine commands|`➜ dkmc ls`
`dkmc create`|Create the docker machine specified by config file|`➜ dkmc create`
`dkmc start`|Start docker machine|`➜ dkmc start`
`dkmc upcerts`|Update/regenerate docker machine's certificates|`➜ dkmc upcerts`
`dkmc load`|Load docker machine's environments|`➜ dkmc load`
`dkcp {command}`|Run any docker-compose commands|`➜ dkcp up`
`dk {command}`|Run any docker commands|`➜ dk ps`
`logs {service}`|Show and follow logs of a specific service|`➜ logs lotus`

## Backlogs

- Setup DNS host alias to connect to services by domain.
- Load config from different file for different projects.
- Ability to define arbitrary command shortcuts.
- Add commands to work with kubernetes.
- Add commands to work with aws or localstack.

## Copyright

- Turtle project avatar is created by [FreeVector.com](https://www.freevector.com/free-cartoon-turtle-vector-18447)
