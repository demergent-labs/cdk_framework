# CDK Framework

The CDK Framework helps CDK authors build CDKs.

## Core Concepts

Start with the AST of your language and compile it into an Abstract Canister Tree (a concept from the CDK Framework). The CDK Framework will then create a Rust canister which can be compiled into Wasm and deployed to the IC.

AST -> ACT -> Rust canister code -> Wasm binary -> IC
