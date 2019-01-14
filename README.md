# Turtle - an interactive shell for micro-services development

[![pipeline](https://gitlab.com/phamlequang/turtle/badges/master/pipeline.svg)](https://gitlab.com/phamlequang/turtle/commits/master) [![license](https://img.shields.io/badge/license-MIT-green.svg)](https://gitlab.com/phamlequang/turtle/blob/master/LICENSE)

Turtle is built with the purpose of helping developers speed up their development process, especially in projects that use micro-services architecture.

One goal of turtle is to provide a set of simple commands to setup, run and manage all services on local machine using docker-compose. This will be useful for writing and running integration or system tests that often need some parts or even the whole system to be up and running. It will also help front-end developers when they develop UI that needs to call APIs on multiple services.

Another goal of turtle is to still allow user to run any normal shell commands when they're inside its shell. It just helps speed up by providing more shortcuts. As of now, some shortcut commands to work with git and docker are supported, kubernetes are comming up. In the future, user will be able to easily define their own shortcut commands.

## Setup

## Commands

The following list contains all supported shortcut commands of the current turtle version.

**Command** | **Description**
--------|-------------
`quit`|Exit turtle shell.
`exit`|Exit turtle shell.
`cd {path}`|Change current working directory to the provided path.
`goto {repository}`|Change current working directory to the provided repository's directory.
`goto {service}`|Change current working directory to the provided service's directory.
`clone`|Clone all repositories listed in the config file.
`clone [repository1] [repository2] ...`|Clone a list of provided repositories.
`pull`|Git pull latest codes on current branch of the repository that contains current working directory.
`pull [repository1] [repository2] ...`|Git pull latest codes on current branch of the provided repositories.
`pull [service1] [service2] ...`|Git pull latest codes on current branch of the provided services' repositories.
`push`|Git push latest codes on current branch of the repository that contains current working directory.
`push [repository1] [repository2] ...`|Git push latest codes on current branch of the provided repositories.
`push [service1] [service2] ...`|Git push latest codes on current branch of the provided services' repositories.
`dkmc {command}`|Run any docker-machine commands.
`dkmc create`|Create a new docker machine as described in the config file.
`dkmc start`|Start the created docker machine.
`dkmc upcerts`|Update or regenerate the created docker machine's certificates.
`dkmc load`|Load the created docker machine's environments.
`dkcp {command}`|Run any docker-compose commands.
`dk {command}`|Run any docker commands.
`use {group}`|Generate or rewrite the working docker-compose file for a group of services.
`start`|Start all services using the generated docker-compose file.
`status`|Show current status of all services.
`stop`|Stop all services using the generated docker-compose file.
`stop [service1] [service2] ...`|Stop the provided services.
`restart`|Restart all services using the generated docker-compose file.
`restart [service1] [service2] ...`|Restart the provided services.
`logs {service}`|Show and follow logs of a specific service.
`build`|Build all services using the command specified in the config file.
`build [service1] [service2] ...`|Build the provided services.
`build [repository1] [repository2] ...`|Build services in the provided repositories.
`build [group1] [group2] ...`|Build services in the provided groups.
`test`|Test all services using the command specified in the config file.
`test [service1] [service2] ...`|Test the provided services.
`test [repository1] [repository2] ...`|Test services in the provided repositories.
`test [group1] [group2] ...`|Test services in the provided groups.
`sh {service}`|Access `/bin/sh` shell of a specific service.
`bash {service}`|Access `/bin/bash` shell of a specific service.

## Backlogs

- Setup DNS host alias to connect to services by domain.
- Load config from different file for different projects.
- Ability to define arbitrary command shortcuts.
- Add commands to work with kubernetes.
- Add commands to work with aws or localstack.

## Copyright

- Turtle project avatar is created by [FreeVector.com](https://www.freevector.com/free-cartoon-turtle-vector-18447)
