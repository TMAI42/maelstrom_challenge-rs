# Rust Echo Node for Maelstrom

## Overview

Welcome to the Rust Echo Node project - a challenge solution aiming to adhere to the guidelines provided by Maelstrom for implementing distributed systems, primarily using Rust. While the challenges are originally crafted with Go in mind, this repository is an adventurous attempt to engage with them using the Rust programming language.

### Note

This project does not leverage the Maelstrom Go library (which simplifies a lot of the node management and message handling mechanics) but instead implements the required functionality using Rust's rich ecosystem of libraries, including Serde for handling JSON data, Anyhow for simplified error handling, and other I/O operations provided by the standard library.

## Installation

Ensure that Rust is installed on your system. If not, you may install it by following the instructions available on the official [Rust website](https://www.rust-lang.org/tools/install).

Clone this repository and navigate into the project directory:

```
git clone git@github.com:TMAI42/maelstrom_challenge-rs.git
```

```
cd maelstrom_challenge-rs
```

Build the project using Cargo:
```
cargo build
```

## Usage

Upon execution, the Rust Echo Node expects JSON messages via STDIN, and will echo the received messages to STDOUT, 
maintaining adherence to the [Maelstrom communication protocol](https://github.com/jepsen-io/maelstrom/blob/main/doc/protocol.md).

## Testing 

To test this realization firstly you should install [Maelstrom](https://github.com/jepsen-io/maelstrom/tree/main).

### Challenge 1: Echo

The initial challenge, "Echo", is fundamentally a stepping stone, or a “getting started” guide to get a grasp of interfacing with Maelstrom. In Maelstrom, we create a node which is an executable binary that negotiates through JSON messages. These messages are received from STDIN (Standard Input) and responses or actions are dispatched to STDOUT (Standard Output). The complete protocol specification can be found on the Maelstrom project site.

#### Getting Started

In this Echo challenge, we emulate an Echo server using Rust, which listens to the incoming JSON messages and echoes them back through the STDOUT. The structure of the messages and the serialization/deserialization methods comply with the JSON formats as specified by Maelstrom.

#### Run echo test

To run the Echo test, you can utilize the following command:
```
./maelstrom test -w echo --bin YOU_PROJECT_DIR/target/debug/maelstrom_challenge-rs --node-count 1 --time-limit 10
```

## Upcoming Challenges Overview

### Challenge #2: Unique ID Generation

Our next step in the challenge series involves the development of a **globally-unique ID generation system**. The crux here is maintaining total availability, ensuring the system continues its operation even amidst network partitions. A tough, yet an exciting puzzle to solve by implementing an ID generator which can uphold its functionality and provide unique IDs, no matter the network circumstances.

### Challenge #3a: Single-Node Broadcast

Transitioning into a more communication-oriented problem, challenge 3a necessitates implementing a **broadcast system** which utilizes gossip protocols to disseminate messages between all nodes in a cluster. Beginning with a single-node broadcast system, this challenge will gradually unfold into a multi-node scenario, exploring the depths of gossiping mechanisms and network communication within a cluster.

### Challenge #4: Grow-Only Counter

Challenge 4 introduces a **stateless, grow-only counter** that adheres to the principles of Maelstrom’s g-counter workload. Unique in its nature, this challenge requires nodes that are reliant on a sequentially-consistent key/value store service provided by Maelstrom, testing the capabilities of stateless computation and counter management in a distributed environment.

### Challenge #5a: Single-Node Kafka-Style Log

Here, the focus shifts to the implementation of a **replicated log service**, akin to Kafka. As we delve into the world of replicated logs, commonly utilized as a message bus or an event stream, we will initially tackle a single-node log system and progressively advance towards a distributed model, examining the nuances and complexities involved in log replication and management.

### Challenge #6a: Single-Node, Totally-Available Transactions

Embarking on a journey into transaction management in distributed systems, this challenge demands the creation of a **key/value store** which enables transactions. Navigating through micro-operations (read & write) and wrestling with weak consistency while maintaining total availability presents an intricate challenge. Beginning with a single-node service, this problem will eventually unfold into a multi-node version, providing a multifaceted look into transaction management in distributed systems.

With each challenge, we're venturing deeper into the realm of distributed systems, embracing the complexities and intricacies these systems present. Stay tuned as we work through these challenges, sharing our journey, solutions, and learnings along the way!





## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.
