# Project
The project is aimed to provide an implementation of the Tribesim program logic as described in the articles [1] and [2] by A. Markov and M. Markov.

[1]: Markov AV, Markov MA. Runaway
brain-culture coevolution as a reason for larger brains:
Exploring the “cultural drive” hypothesis by computer
modeling. Ecol Evol. 2020;10:6059–6077. https://doi.
org/10.1002/ece3.6350

[2]: Markov AV, Markov MA. Coevolution of Brain, Culture, and Lifespan:
Insights from Computer Simulations. Biochemistry. DOI: 10.1134/S0006297921120014


## Disclamer
The project is in the initial stage of its development and is subject to significant change.

## Development plan

1. All simulation parameters should be defined in a structure, which can be loaded from a configuration file or constructed from a http request
1. A separate metadata database storing the simulation parameters for the puspose of annotating the run statistics should be supported.
1. The Tribesim core implementation should be moved to a separate crate.
1. A web application providing basic UI allowing supplying parameters and initiating runs should be implemented.
1. The feature set of the 2021 article should be implemented.

# Project Structure

The project follows a **modular** design, separating the **simulation** logic (agents, groups, memetics) from the **model** logic (cultural processes, reproduction, population dynamics) and the **database** layer. Here’s an overview of the major files and directories:

```
.
├── src/
│   ├── simulation/
│   │   ├── agent.rs
│   │   ├── group.rs
│   │   ├── memetics.rs
│   │   └── mod.rs
│   ├── model/
│   │   ├── culture.rs
│   │   ├── distribution.rs
│   │   ├── population.rs
│   │   ├── reproduction.rs
│   │   └── mod.rs
│   ├── db/
│   │   ├── clickhouse_client.rs
│   │   └── mod.rs
│   ├── lib.rs
│   └── main.rs
└── Cargo.toml
```

## `src/simulation/`

- **`agent.rs`**  
  Defines the `Agent` struct, encapsulating properties like memory capacity, efficiencies, resources, and references to cultural traits. This file typically handles per-agent logic (e.g., initialization, resource manipulation).

- **`group.rs`**  
  Declares the `Group` struct, containing a collection of agents and metadata (like `group_id`, configuration, etc.). Group-level behaviors—such as group splitting, membership changes—may also appear here or in associated functions.

- **`memetics.rs`**  
  Implements the logic for meme creation, teaching, and learning. This might include functions for generating new memes, selecting memes to pass along, or applying trick/teaching/learning processes.

- **`mod.rs`**  
  A module file re-exporting or organizing `agent.rs`, `group.rs`, and `memetics.rs`, so that other parts of the code can simply refer to `simulation::Agent`, `simulation::Group`, etc.

## `src/model/`

- **`culture.rs`**  
  Houses detailed cultural logic: how memes are stored in an agent, how memory usage is tracked, or how cultural “efficiencies” (learning/teaching) are updated.

- **`distribution.rs`**  
  Functions or utilities for distributing resources (or other quantities) among agents, often used after hunting or group-level resource acquisition steps.

- **`population.rs`**  
  Possibly includes higher-level population dynamics, such as the orchestration of births/deaths, group expansions, or multi-group interactions.

- **`reproduction.rs`**  
  Logic for agent reproduction: inheritance of traits, mutation, costs associated with child brain volume, resource checks, and child creation.

- **`mod.rs`**  
  Acts as an entry point for the entire `model` module, re-exporting the above submodules so they can be accessed as `model::culture`, `model::reproduction`, etc.

## `src/db/`

- **`clickhouse_client.rs`**  
  Contains all ClickHouse-related I/O: establishing a connection client, constructing insertion logic (batched or otherwise), and possibly schema definitions or data struct mappings for writing simulation stats to the database.

- **`mod.rs`**  
  A module file that re-exports items from `clickhouse_client.rs` and organizes the `db` code for simpler imports in the rest of the application.

## Top-Level Files

- **`src/lib.rs`**  
  Defines the library crate for your project, typically pulling together all submodules and re‐exporting them. Other code or binaries can rely on `use your_crate_name::*` to access these shared components.

- **`src/main.rs`**  
  The main **entry point** (binary) for running the simulation. It typically sets up the environment, parses command-line arguments or config, creates the initial population/groups, and coordinates the simulation loop (e.g., year-by-year updates). Also orchestrates any final data exports or logs to the DB.

## Preparation
The application requires access to a Clickhouse database to save statistics of its simulation runs. The SQL statements for the database deployment are present in the `sql/clickhouse/` project directory.

## Building & Running

- **Build** the project:
  ```bash
  cargo build --release
  ```

- **Run** the project:
  ```bash
  export CLICKHOUSE_URL="http://host:port/"
  export CLICKHOUSE_USER=tribesim
  export CLICKHOUSE_PASSWORD=YourSecurePassw0rd!
  export CLICKHOUSE_DB=tribesim
  ./target/release/tribesim
  ```
