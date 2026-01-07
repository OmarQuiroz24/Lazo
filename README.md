# Lazo
Lazo is a unified engineering platform designed for the orchestration, simulation, and real-time control of physical systems. Built on Rust, the system bridges the gap between mathematical modeling, hardware data acquisition, and complex workflow automation.
The Lazo ecosystem allows developers to transition seamlessly from rapid prototyping on low-cost microcontrollers to the deployment of robust industrial infrastructure under a single software standard.

# Core Engine Architecture (Kernel)
The Lazo core is engineered around the principles of memory safety and temporal determinism, leveraging Rust's native capabilities to guarantee stability in critical control loops.
- Asynchronous Execution Model: Utilizes an actor-based runtime where each system component is an isolated process. This prevents failures in non-critical layers from compromising the execution of hardware control nodes.
- High-Efficiency Data Management: Implements Zero-Copy data transfer mechanisms, allowing sensor signals and actuator commands to flow between nodes with minimal overhead.
- Deterministic Scheduler: The engine prioritizes signal processing and hardware communication over UI tasks, ensuring constant sampling rates and low jitter.

# Visual Orchestration & Automation
Lazo adopts a node-based visual orchestration paradigm, optimized for control system dynamics.
- Dynamic Flow Canvas: Build control logic by interconnecting functional blocks on an infinite canvas. Automate processes by logically linking inputs, processors, and outputs.
- Workflow Automation: Supports conditional workflows, alerts, API integrations, and safety protocols—all managed visually.
- Configuration-as-Code (TOML): Graph structures, connections, and node configurations are stored in human-readable TOML files, making projects easy to audit, version-control with Git, and portable.

# Hardware Abstraction Layer
Lazo acts as a universal standard with a robust Hardware Abstraction Layer for physical world communication.
- Lazo Bridge: A low-latency communication bridge that enables automatic device discovery (MCUs and SBCs) and peripheral configuration via capability descriptors.
- Deterministic Acquisition: Uses buffering and source-side timestamping to ensure data from microcontrollers is processed in exact chronological order.

# Extensibility & Scripting
While the core is built in Rust for performance, Lazo is fully extensible and polyglot
- Custom Logic Nodes: Inject proprietary logic using high-level languages like Python or Rust for data processing, AI, DSP, or system integration.
- Execution Isolation: Scripts run in controlled environments that communicate safely with the Rust kernel.

# Interface & Visualization
Built on a hybrid architecture, Lazo-Studio combines Rust's power with modern web technologies.
- Real-Time Visualization: Hardware-accelerated rendering for telemetry, oscilloscopes, and control dashboards.
- UI/Kernel Decoupling: The graphical interface runs independently from the calculation engine to ensure control loop stability.

# Vision
Lazo aims to provide a deterministic framework for cyber-physical systems, eliminating the friction between high-level abstraction and low-level hardware control. It empowers even low-cost microcontrollers with advanced capabilities like machine learning, digital signal processing, and IoT connectivity, all through a visual, accessible, and safe platform that sets a new standard for mission-critical automation and orchestration.
