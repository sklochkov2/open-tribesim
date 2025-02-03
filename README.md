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

# Compilation and usage
Rust 1.84 is required. Older versions **may** work as well, but are not supported.

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
  export SIM_CONFIG=examples/cfg.json
  export MYSQL_URL="mysql://user:MySecurePassw0rd@localhost:3306/tribesim"
  ./target/release/tribesim                 # Launch the simulation in one-shot mode.
  ./target/release/tribesim --launch-server # Launch web server providing Tribesim REST API.
  ```

# JSON Configuration Format

This project allows you to **configure** various simulation parameters (mutation probabilities, agent properties, group limits, etc.) using a **JSON** file. Below is an **example** JSON layout corresponding to the `SimConfig` struct and its sub-structures.

```json
{
  "mutation_config": {
    "mem_mutation": {
      "probability": 0.05,
      "magnitude_std": 1.0
    },
    "learning_mutation": {
      "probability": 0.02,
      "magnitude_std": 0.3
    },
    "teaching_mutation": {
      "probability": 0.01,
      "magnitude_std": 0.2
    }
  },
  "agent_config": {
    "base_brain_volume": 20.0,
    "mem_cost": 1.0,
    "death_prob_multiplier": 0.002
  },
  "meme_config": [
    {
      "meme_kind": "Hunting",
      "probability": 0.15,
      "size": {
        "min": 0.5,
        "max": 2.5
      },
      "effect": {
        "min": 0.1,
        "max": 0.6
      }
    },
    {
      "meme_kind": "Trick",
      "probability": 0.10,
      "size": {
        "min": 0.3,
        "max": 1.0
      },
      "effect": {
        "min": 0.2,
        "max": 1.5
      }
    }
  ],
  "group_config": {
    "max_size": 150
  },
  "epoch": 5000,
  "resources": 1000.0
}
```

### Explanation

- **`mutation_config`**  
  - Defines how different traits (memory, learning, teaching) can mutate.  
  - Each **`MutationParams`** block has a `probability` of mutation and a `magnitude_std` controlling the size of the mutation step.

- **`agent_config`**  
  - **`base_brain_volume`**: Base brain volume for all agents.  
  - **`mem_cost`**: How much brain volume scales with memory usage.  
  - **`death_prob_multiplier`**: Factor determining age-based death probability.

- **`meme_config`**  
  - A list of **`MemeConfig`** entries, each describing a possible meme type (`meme_kind` can be `"Hunting"`, `"Learning"`, `"Teaching"`, `"Trick"`, or `"Useless"`).  
  - **`probability`** indicates how likely a new meme of this type is to appear (per time step, or however your simulation uses it).  
  - **`size`** and **`effect`** define min/max ranges for meme size or effect levels, used in random sampling.

- **`group_config`**  
  - **`max_size`** sets the limit at which a group splits.

- **`epoch`**  
  - The total number of discrete time steps (years, generations, etc.) to simulate.

- **`resources`**  
  - How many resources are available in total (for the entire simulation step), or some other global supply measure.

### Usage

1. **Create** a JSON file (e.g. `config.json`) with the contents shown above (adjusting values as desired).
2. **Load** it in your Rust code using something like:
   ```rust
   let sim_config: SimConfig = load_config_from_json("config.json")
       .expect("Failed to load simulation config");
   ```
   (assuming you’ve implemented or used a helper like `serde_json::from_reader`).

3. **Run** the simulation with these parameters. You can store or retrieve multiple config files for different scenarios, easily versioning your experiments.

This JSON-based approach makes it straightforward to **edit** or **share** simulation settings without recompiling the code.

# Project Structure

The project follows a **modular** design, separating the **simulation** logic (agents, groups, memetics) from the **model** logic (cultural processes, reproduction, population dynamics) and the **database** layer. Here’s an overview of the major files and directories:

```
.
├── Cargo.toml
├── src
│   ├── api
│   │   ├── api_server.rs
│   │   ├── model.rs
│   │   └── mod.rs
│   ├── cli
│   │   ├── args.rs
│   │   └── mod.rs
│   ├── config
│   │   ├── config.rs
│   │   ├── file.rs
│   │   └── mod.rs
│   ├── db
│   │   ├── clickhouse_client.rs
│   │   ├── mod.rs
│   │   └── mysql_client.rs
│   ├── lib.rs
│   ├── main.rs
│   ├── model
│   │   ├── culture.rs
│   │   ├── distribution.rs
│   │   ├── mod.rs
│   │   ├── population.rs
│   │   └── reproduction.rs
│   ├── runtime
│   │   ├── mod.rs
│   │   ├── run_sim.rs
│   │   └── statistics.rs
│   ├── simulation
│   │   ├── agent.rs
│   │   ├── group.rs
│   │   ├── memetics.rs
│   │   └── mod.rs
│   └── utils.rs
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

- **`mysql_client.rs`**
  Contains all MySQL-related I/O and all logic required to save simulation metadata.

- **`mod.rs`**  
  A module file that re-exports items from `clickhouse_client.rs` and organizes the `db` code for simpler imports in the rest of the application.

## `src/config`

- **`config.rs`**
  Contains all structs that constitute the simulation configuration.

- **`file.rs`**
  Contains helper functions that savethe simulation configuration to a JSON file and load it.

## `src/cli`

- **`args.rs`**
  Contains the Args struct which encapsulates all the command line arguments.

## `src/runtime`

- **`run_sim.rs`**
  Contains the simulation loop which executes all changes on the simulated groups for all the simulated "years".

- **`statistics.rs`**
  Contains the functions used for simulation instrumentation; they aggregate the data for subsequent insertion into Clickhouse.

## `src/api`

- **`api_server.rs`**
  Contains implementation of the tribesim REST API.

- **`model.rs`**
  Contains all the data structures sent or received by the REST API.


## Top-Level Files

- **`src/lib.rs`**  
  Defines the library crate for your project, typically pulling together all submodules and re‐exporting them. Other code or binaries can rely on `use your_crate_name::*` to access these shared components.

- **`src/main.rs`**  
  The main **entry point** (binary) for running the simulation. It typically sets up the environment, parses command-line arguments or config, creates the initial population/groups, and coordinates the simulation loop (e.g., year-by-year updates). Also orchestrates any final data exports or logs to the DB.

- **`src/utils.rs`**
  Used to store generic helper functions. At the moment, it contains only the `generate_uuid()` function.

