# Titles and stuff I think
## Description
A simple CLI program to setup a simple and minimal java project.

## Problems it solves for me
- Gradle initial projects are bloated with too many files.
- Setting up a java project by hand is error prone.
    - spelling mistakes
    - nesting mistakes
- Setting up a java project by hand is not fun.

## Features
- [x] Generate full project structure somewhat reasonably.
- [x] Generate files like `.gitignore`, `build.gradle.kt`, ...
- [x] Populates generated files with proper defaults.
- [x] Setup `gradle`.
- [x] Optional flag for project name, default: app. `-n`
- [x] Optional flag for project domain, default: org.example. `-d`
- [x] Optional flag for `preset` dependencies, defaults: "simple". `-p`
- [ ] Optional flag for `git` repository, defaults: false. `-g`
- [ ] Optional flag for making a modular java structure, default: false. `-m`
- [ ] Optional flag for prompting for all of the optional flags. `-a` 🤡

## Usage
```sh
newj [PATH] -n|--name [NAME] -d|--domain [DOMAIN] -p|--preset [simple|mc-plugin|spring]
```
##### if `x` is not provided:
PATH: default to present working directory <br>
NAME (name of your project): prompts you<br>
DOMAIN (domain used for your project): prompts you <br>
PRESET: defaults to "simple".

That being said, you can only type `newj` for a project in the present directory and prompts for name and domain.
```sh 
newj
```
## Default project structure
name: app (default) <br>
domain: org.example (default)

```
app 
└── src 
    ├── main 
    │   └── java 
    │       └── org 
    │           └── example 
    │               └── app 
    │                   └── App.java 
    ├── test 
        └── java 
            └── org 
                └── example 
                    └── app 
    ├── .gitignore 
    └── build.gradle.kt 
```
