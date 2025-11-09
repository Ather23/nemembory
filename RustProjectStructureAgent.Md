# Rust Layered Application Scaffolder

This document outlines the process for using an AI assistant to automatically generate the structure for a layered Rust application using a Cargo workspace.

## How It Works

The AI assistant will guide you through a series of questions to understand your desired project structure. Based on your answers, it will generate a complete, step-by-step guide with all the necessary commands and file content to create your project.

## Usage

To start, simply tell the assistant you want to create a layered Rust application. The assistant will then ask you the following questions:

1.  "How many layers would you like for your application?"
2.  "What are the names of these layers? Please list them, separated by commas (e.g., `my-app, core-logic, data-access`)."
3.  "Which of the layers you named should be the main application entry point (i.e., the binary crate)?"

### Example Interaction

Here is an example of how you might interact with the assistant.

**Your Request:**

> "I want to create a layered Rust application with a web API, business logic, and a database layer."

**Assistant's Questions:**

> 1. How many layers would you like for your application?
> 2. What are the names of these layers? Please list them, separated by commas.
> 3. Which of the layers you named should be the main application entry point?

**Your Answers:**

> 1. 3
> 2. `api`, `logic`, `db`
> 3. `api`

### Generated Output

Based on the answers above, the assistant will provide a detailed guide similar to the following:

---

_Here are the steps to create your layered Rust application:_

---

### **1. Create the Workspace Root**

```sh
mkdir my-layered-app
cd my-layered-app
```

### **2. Create the Crates (Layers)**

Run these commands to create your application and library crates.

```sh
cargo new api
cargo new --lib logic
cargo new --lib db
```

### **3. Define the Workspace**

Create a `Cargo.toml` file in the root directory (`my-layered-app/Cargo.toml`) with the following content:

```toml
[workspace]
resolver = "2"
members = [
    "api",
    "logic",
    "db",
]
```

### **4. Add Dependencies Between Layers**

To use code from the `logic` layer within your `api` layer, add it as a dependency in `api/Cargo.toml`:

```toml
[dependencies]
logic = { path = "../logic" }
```

To use the `db` layer within your `logic` layer, add it as a dependency in `logic/Cargo.toml`:

```toml
[dependencies]
db = { path = "../db" }
```

---
