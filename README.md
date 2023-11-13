# Rust Design Patterns: Strategy and Command

This repository contains Rust implementations of the Strategy and Command design patterns. These patterns are useful for encapsulating algorithms and operations, making them interchangeable and more manageable.

## Strategy Pattern

### Overview

The Strategy pattern is used to define a family of algorithms, encapsulate each one, and make them interchangeable. This pattern lets algorithms vary independently from the clients that use them.

### Structure

- **Strategy Trait**: An interface for executing an operation.
- **Concrete Strategies**: Implementations of the Strategy interface.
- **Context**: Maintains a reference to a Strategy object.

### Implementation

- `trait Strategy`
- `struct ConcreteStrategyA`
- `struct ConcreteStrategyB`
- `struct Context`

### Usage

Create instances of concrete strategies and pass them to a context. The context delegates the strategy execution.

## Command Pattern

### Overview

The Command pattern encapsulates all information needed to perform an action or trigger an event. This encapsulation allows for more complex architectures like queued and logged operations.

### Structure

- **Command Trait**: An interface for executing commands.
- **Concrete Commands**: Specific commands implementing the Command interface.
- **Invoker**: Holds a command and can invoke its execution.

### Implementation

- `trait Command`
- `struct ConcreteCommandA`
- `struct ConcreteCommandB`
- `struct Invoker`

### Usage

Instantiate concrete command objects and pass them to an invoker. The invoker triggers these commands.

## Tests

Tests are included for both patterns to ensure functionality. Use `cargo test` to run these tests.

## Getting Started

- Clone the repository.
- Navigate to the project directory.
- Run `cargo build` to compile.
- Run `cargo test` to execute the tests.


