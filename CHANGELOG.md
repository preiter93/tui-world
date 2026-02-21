# Changelog

All notable changes to this project will be documented in this file.

## [0.1.4] - 2026-02-21

### 🐛 Bug Fixes

- *(ci)* Fail lint on warning

### 📚 Documentation

- *(readme)* Clean up example by removing unused imports
- *(examples)* Use display compact
- Document public structs
- Add documentation for lib.rs
- *(readme)* Add badges
- *(readme)* Remove WIP warning
- *(readme)* Center title
- *(readme)* Split example in 2
- *(readme)* Unify docs between readme and lib.rs

### ⚙️ Miscellaneous Tasks

- *(examples)* Simplify help example

## [0.1.3] - 2026-02-18

### 🚀 Features

- *(kb-display)* Add a compact display version

### 🐛 Bug Fixes

- *(keybindings)* Display Shift+letter with letter as lowercase

### 🔧 Refactor

- *(event)* Rename Event to InputEvent to avoid naming conflicts
- *(keybindings)* Group DisplayInfo by action

## [0.1.2] - 2026-02-16

### 🚀 Features

- *(keybindings)* Auto-add SHIFT modifier for uppercase char conversions

## [0.1.1] - 2026-02-08

### 🚀 Features

- *(keybindings)* Simplify keybinding context
- *(pointer)* Merge layout and mouse into pointer
- *(pointer)* Add drag event
- *(focus)* Add "new" constructor
- *(focus)* Add is_focused method

### 🐛 Bug Fixes

- *(keybindings)* Prevent duplicate keybindings

### 📚 Documentation

- *(example)* Improve todo example
- *(example)* Add selectable text example
- *(readme)* Simplify example
- *(example)* Update help example to show global as last

### 🔧 Refactor

- *(keybindings)* Rename catch_all to bind_any
- *(todo)* Setup click handlers in render
- *(example)* Simplify todo example and add help example
- *(pointer)* Pass area to click handlers

### ⚙️ Miscellaneous Tasks

- *(ci)* Add release workflow

## [0.1.0] - 2026-01-17

### 🚀 Features

- Map " " to "Space"


