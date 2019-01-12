# Turtle - an interactive shell for micro-services development

[![pipeline](https://gitlab.com/phamlequang/turtle/badges/master/pipeline.svg)](https://gitlab.com/phamlequang/turtle/commits/master) [![license](https://img.shields.io/badge/license-MIT-green.svg)](https://gitlab.com/phamlequang/turtle/blob/master/LICENSE)

Turtle is built with the purpose of helping developers speed up their development process, especially in projects that use micro-services architecture.

One goal of turtle is to provide a set of simple commands to setup, run and manage all services on local machine using docker-compose. This will be useful for writing and running integration or system tests that often need some parts or even the whole system to be up and running. It will also help front-end developers when they develop UI that needs to call APIs on multiple services.

Another goal of turtle is to still allow user to run any normal shell commands when they're inside its shell. It just helps speed up by providing more shortcuts. As of now, some shortcut commands to work with git and docker are supported, kubernetes are comming up. In the future, user will be able to easily define their own shortcut commands.

## Setup

## Commands

## Backlogs

- Ability to define arbitrary patterns for path replacement.
- Better UI for status command.
- Load config from different file for different projects.
- Ability to define arbitrary command shortcuts.
- Add commands to work with kubernetes.
- Add commands to work with aws or localstack.
