# Turtle - an interactive shell for micro-services development

[![pipeline](https://gitlab.com/phamlequang/turtle/badges/master/pipeline.svg)](https://gitlab.com/phamlequang/turtle/commits/master) [![license](https://img.shields.io/badge/license-MIT-green.svg)](https://gitlab.com/phamlequang/turtle/blob/master/LICENSE)

Turtle is built with the purpose of helping developers speed up their development process, especially in projects that use micro-services architecture.

One goal of turtle is to provide a set of simple commands to setup, run and manage all services on local machine using docker-compose. This will be useful for writing and running integration or system tests that often need some parts or even the whole system to be up and running. It will also help front-end developers when they develop UI that needs to call APIs on multiple services.

Another goal of turtle is to still allow user to run any normal shell commands when they're inside its shell. It just helps speed up by providing more shortcuts. As of now, some shortcut commands to work with git and docker are supported, kubernetes are comming up. In the future, user will be able to easily define their own shortcut commands.

## Setup

## Commands

The following list contains supported shortcut commands of current turtle version. All example commands are using the [sample config file](https://gitlab.com/phamlequang/turtle/blob/master/etc/config.toml) in `etc` folder.

**Command** | **Description** | **Example**
--------|-------------|--------
`quit`|Exit turtle shell.|`➜ quit`
`exit`|Exit turtle shell.|`➜ exit`
`cd {path}`|Change current working directory to the provided path.|`➜ cd ~/projects/flowers/camellia/`
`goto {repository}`|Change current working directory to the provided repository's directory.|`➜ goto flowers`
`goto {service}`|Change current working directory to the provided service's directory.|`➜ goto lotus`
`clone`|Clone all repositories listed in the config file.|`➜ clone`
`clone [repository1] [repository2] ...`|Clone a list of provided repositories.|`➜ clone flowers trees`
`pull`|Git pull latest codes on current branch of the repository that contains current working directory.|`➜ pull`
`pull [repository1] [repository2] ...`|Git pull latest codes on current branch of the provided repositories.|`➜ pull flowers trees`
`pull [service1] [service2] ...`|Git pull latest codes on current branch of the provided services' repositories.|`➜ pull camellia lotus`
`push`|Git push latest codes on current branch of the repository that contains current working directory.|`➜ push`
`push [repository1] [repository2] ...`|Git push latest codes on current branch of the provided repositories.|`➜ push flowers trees`
`push [service1] [service2] ...`|Git push latest codes on current branch of the provided services' repositories.|`➜ push camellia lotus`
`dkmc {command}`|Run any docker-machine commands.|`➜ dkmc ls`
`dkmc create`|Create a new docker machine as described in the config file.|`➜ dkmc create`
`dkmc start`|Start the created docker machine.|`➜ dkmc start`
`dkmc upcerts`|Update or regenerate the created docker machine's certificates.|`➜ dkmc upcerts`
`dkmc load`|Load the created docker machine's environments.|`➜ dkmc load`
`dkcp {command}`|Run any docker-compose commands.|`➜ dkcp up`
`dk {command}`|Run any docker commands.|`➜ dk ps`
`use {group}`|Generate or rewrite the working docker-compose file for a group of services.|`➜ use all`
`start`|Start all services using the generated docker-compose file.|`➜ start`
`status`|Show current status of all services.|`➜ status`
`stop`|Stop all services using the generated docker-compose file.|`➜ stop`
`stop [service1] [service2] ...`|Stop the provided services.|`➜ stop redis lotus`
`restart`|Restart all services using the generated docker-compose file.|`➜ restart`
`restart [service1] [service2] ...`|Restart the provided services.|`➜ restart redis lotus`
`logs {service}`|Show and follow logs of a specific service.|`➜ logs lotus`
`build`|Build all services using the command specified in the config file.|`➜ build`
`build [service1] [service2] ...`|Build the provided services.|`➜ build camellia lotus`
`build [repository1] [repository2] ...`|Build services in the provided repositories.|`➜ build flowers`
`build [group1] [group2] ...`|Build services in the provided groups.|`➜ build svc`
`test`|Test all services using the command specified in the config file.|`➜ test camellia lotus`
`test [service1] [service2] ...`|Test the provided services.|`➜ test camellia lotus`
`test [repository1] [repository2] ...`|Test services in the provided repositories.|`➜ test flowers`
`test [group1] [group2] ...`|Test services in the provided groups.|`➜ test svc`
`sh {service}`|Access `/bin/sh` shell of a specific service.|`➜ sh lotus`
`bash {service}`|Access `/bin/bash` shell of a specific service.|`➜ bash lotus`

## Backlogs

- Setup DNS host alias to connect to services by domain.
- Load config from different file for different projects.
- Ability to define arbitrary command shortcuts.
- Add commands to work with kubernetes.
- Add commands to work with aws or localstack.

## Copyright

- Turtle project avatar is created by [FreeVector.com](https://www.freevector.com/free-cartoon-turtle-vector-18447)
