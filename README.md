# CDK Framework

The CDK Framework helps CDK authors build CDKs for the [Internet Computer](https://internetcomputer.org/).

## Contents

- [Installation](#installation)
- [Abstract Canister Tree (ACT)](#abstract-canister-tree)
- [Parts of the ACT](#parts-of-the-abstract-canister-tree)
- [Advanced Usage](#advanced-usage)
  - [Inline Names](#inline-names)
  - [Flatten](#flatten)
  - [ToTypeAnnotation](#to-type-annotation)
  - [Traits](#traits)

## Installation

The CDK Framework is not currently available on crates.io. In order to use it in your CDK project add the following dependencies to your Cargo.toml:

> **Note**
> For rev use the hash of current commit on the main branch

```toml
[dependencies]
cdk_framework = { git = "https://github.com/demergent-labs/cdk_framework", rev = "08a74a4077d0981acde2b0fc714e877bba659b9e" }
quote = "1.0.21"
proc-macro = "1.0.43"
```

## Abstract Canister Tree

The Abstract Canister Tree (ACT) is a concept that the CDK Framework introduces to streamline the creations of a CDK.

The idea behind the CDK framework is to help convert a canister written in any language to a Rust canister that can then be compiled to WASM and deployed on the IC. To create a Canister your CDK should start with the AST of your language and compile it into an Abstract Canister Tree. The CDK Framework will then generate the contents of a lib.rs file which you can add to a Rust canister which can then be compiled into Wasm and deployed to the IC.

AST -> ACT -> Rust canister code -> Wasm binary -> IC

> **Note**
> While the CDK framework will help to generate the lib.rs files for the canisters your CDK will build, your CDK will still need to provide all of the other parts of teh Rust canister.

To generate the Rust token stream for the lib.rs you simply need to construct an `AbstractCanisterTree`

For example

```rust
fn main() {
    // TODO generate AST representation of the canister you are parsing

    let cdk_name = "YOUR_CDK_NAME".to_string() // For example: "kybra".to_string()

    let funcs;          // TODO build funcs,
    let records;        // TODO build records,
    let tuples;         // TODO build tuples,
    let type_aliases;   // TODO build type aliases,
    let variants;       // TODO build variants,

    let candid_types = CandidTypes {
        funcs,
        records,
        tuples,
        type_aliases,
        variants,
    };

    let heartbeat_method;       // TODO build heartbeat method,
    let init_method;            // TODO build init method,
    let inspect_message_method; // TODO build inspect method,
    let post_upgrade_method;    // TODO build post upgrade method,
    let pre_upgrade_method;     // TODO build pre upgrade method,
    let query_methods;          // TODO build query methods,
    let update_methods;         // TODO build update methods,

    let canister_methods = CanisterMethods {
        heartbeat_method,
        init_method,
        inspect_message_method,
        post_upgrade_method,
        pre_upgrade_method,
        query_methods,
        update_methods,
    };

    let external_canisters; // TODO build external canisters

    let guard_functions;    // TODO build guard functions

    let header; // TODO generate any Rust code your CDK needs to come at the beginning of the generated lib file
    let body;   // TODO generate any Rust code your CDK needs to have in the body of the generated lib file
    let try_from_vm_value_impls; // TODO generate try into vm value impls,
    let try_into_vm_value_impls; // TODO generate try from vm value impls,

    let vm_value_conversion = VmValueConversion {
        try_from_vm_value_impls,
        try_into_vm_value_impls,
    };

    let keywords;   // TODO generate a list of keyword from your CDK's language
    // For example vec!["for", "if", "int", "import", "bool", "while", etc...]

    let lib_file = AbstractCanisterTree {
        cdk_name,
        candid_types,
        canister_methods,
        external_canisters,
        guard_functions,
        header,
        body,
        vm_value_conversion,
        keywords,
    }.to_token_stream().to_string()

    // TODO write contents of lib_file to a file

    // TODO generate cargo.toml

    // TODO build your generated Rust project and deploy to replica
}

```

## Parts of the Abstract Canister Tree

With the high level overview of the ACT in mind, let's now explore each part required to build the ACT.

- [Candid Types](#candid-types)
  - [Members and Elems](#members-and-elems)
- [Canister Methods](#canister-methods)
  - [Params](#params)
  - [Return Type](#return-type)
  - [Function Body](#body)
- [External Canisters](#external-canisters)
- [Guard Functions](#guard-functions)
- [Custom Rust](#custom-rust)
- [Generated Cargo.toml](#your-cdk)

### Candid Types

[Candid](https://internetcomputer.org/docs/current/developer-docs/build/candid/candid-intro) is an interface description language created by [DFINITY](https://dfinity.org/). It defines interfaces between services (in our context canisters), allowing canisters and clients written in various languages to easily interact with each other.

Besides the candid types that are part of [canister methods](#canister-methods) or [external canisters](#external-cansiters), any complex candid types will need to be provided to the ACT so that they can be fully defined in the generated lib files. The `act::CandidTypes` struct enumerates all of the types you maybe need to provide. Do not confuse this with `act::node::CandidType` which is a enum with all of the possible candid types.

> **Note**
> The primitive types and other types such as arrays and opts do not need this additional definition because the CDK framework already knows how to define them, therefore they are not included in `act::CandidTypes`.

### Members and Elems

Records and Variant have members that are just a name and a `CandidType`. Similarly Tuples have elems that are just a wrapper for `CandidType`. These structs will become important in the [Advanced Usage Section](#advanced-usage) when we start discussing [inline names](#inline-names).

### Canister Methods

The `act::CanisterMethods` struct has all of the canister methods that your canister may need to define. All of them are optional. If the canister you are parsing doesn't need one, you won't need to provide it to the CDK framework. As with the [candid types](#candid-types), don't confuse `act::CanisterMethods` (a collector for all of the canister methods that a canister defines) with `act::node::CanisterMethod` (an enum of all possible types of canister methods).

All of the system canister methods are very similar. They will all have some combination of the following things: a list of [params](#params), a [body](#body), and/or a [guard function](#guard-functions) name.

#### Updates and Queries

Updates and queries have a little more information that they need. In addition to a [guard function](#guard-functions) name, a [body](#body), and a list of [params](#params), the CDK Framework will need to know if the method is asynchronous, if it's manual, the methods's [return type](#return-type), the name of the method, and the name of your CDK.

All of this information is encapsulated in the `act::node::canister_method::QueryOrUpdateDefinition` struct.

#### Params

The params are simple structs that have the name of the param, and the `CandidType` of that param.

#### Return Type

The return type is simply a wrapper around `CandidType` that is used for naming any inline dependencies. It's `new()` function will take care of everything for you. `act::node::ReturnType::new(candid_type)`

#### Body

The body will be a Rust token stream that will determine how the function interacts with your CDK's VM.

### External Canisters

External Canisters are simply the name of the external canister and a list of methods that that canister has. The methods are represented by `act::node::external_canister::Method`, which is simply a name, list of [params](#params), and a [return type](#return-type)

### Guard Functions

Guard functions are special types of functions that run before a canister method is run and determine if that canister method will be run or not. To create a Guard Function you just need a name, and a [body](#body). Each canister method that wants to use this guard function will need specify this guard function's name when creating it.

### Custom Rust

These are parts of the lib file that the CDK framework is unable to generalize. For example it might include code to import and set up your CDK's vm. The Rust parts are, header, body (not to be confused with a [function/method body](#body)), and the try into and try from vm value impls.

### Generated Cargo.toml

Please make sure that as you generate your own Cargo.toml file for each Rust canister that it has at least the following dependencies.

```toml
[dependencies]
ic-cdk = { version = "0.6.8", features = ["timers"] }
ic-cdk-macros = "0.6.8"
candid = "0.8.4"
```

## Advanced Usage

### Inline Names

If you CDK's language supports inline types then the CDK framework can handle them. For example if you had an inline record for the return type of a query method, then you would create an `act::node::candid::Record` and set it as the return type for that query method, and then the CDK framework will find that inline type and make sure that it declared in the generated lib file. The name it creates will be based on where it is in the act. For the return type of a query method called `hello_world` the generated return type identifier would be `_InlineHelloWorldReturnType`. If there was an inline parameter called `greeting`, on that function then it would receive the name `_InlineHelloWorldGreeting`. The cdk_framework should take care of generating most of those names for you. However should you need to generate a matching inline name in your CDK then the framework exposes it's methods to help you consistently name everything.

There are two ways places where inline names are generated for an act node. The first place is on the node itself. In this case it will need the inline name you want to give it. For example, if you were trying to recreate the return type from above you would get the `CandidType` from the `ReturnType`, and call `to_type_annotation()` passing in "HelloWorldReturnType' as the `inline_name`. The CDK framework will automatically prepend \_Inline.

The second place was designed to help create consistency among the various things that have return types, params or members, so that you don't have `_InlineHelloWorldReturns` in some places and `_InlineHelloWorldReturnType` in other places. To access these you would call `to_type_annotation()` directly on the `Param`, `ReturnType`, or `Member`. In this case you will only pass in the name of the function, func, record, variant, or tuple that has the param, return type, or member, and it will append the appropriate suffix for you to ensure consistency.

### Flatten

The `flatten()` function should mostly be used by the CDK framework as a way to collect inline types and flatten them to a list of Rust token streams to be added to the generated lib.rs file. You may find you need to call this for super-cdk features. The `flatten()` function will return a list of token_streams. It will require an inline name which will be determined by where you are calling it. Lets expand the example in [inline name](#inline-names). If for whatever reason you found yourself needing to get all of the token streams for all of the inline types declared as part of that `hello_world` canister method's return type, you would call `flatten()` on the return type's candid value and pass in "HelloWorldReturnType" as the `inline_name`, or, alternatively, you could use the preferred method of calling `flatten()` directly on the return type and simply pass in "HelloWorld" as the `parent_name` or `function_name`. While the latter is preferred you may need to flatten a candid type that is not a return type, param, or member, and you will need to know how it behaves in the general case.

The flatten function is implemented on all act nodes and comes from the `Declare` trait.

### ToTypeAnnotation

The `ToTypeAnnotation` is implemented on all variations of `CandidType` and provides the `to_type_annotation()` method. It gives you the name of a type as it would appear in a type annotation for a param, or a return type for example. It follows the same naming scheme discussed in [inline names](#inline-names) and [flatten](#flatten).

### Traits

The CDK framework defines a handful of traits as helpers for collecting inline types, ensuring consistency, and making the code look a little neater. Very briefly they are,
`HasInlines` which is a helper for collecting inline types. It is implemented on two of the other traits, `IsCallable` and `HasMembers`, which anything that is callable or has members can implement to make it easier to collect the inline types for that node. For example the `external_canister::Method` implements `IsCallable`. That implementation will allow `IsCallable` to know how to access its return type and params, so that you can call `flatten_inlines` (from `HasInlines`) directly on the `external_canister::Method` to get all of the inline types for the `Member`.

The `HasInlineName` trait is just to make a consistent interface for things that may have a special inline name. For example the `ReturnType` needs to append "ReturnType" to the end of all its names. The `HasInlineName` implementation for `ReturnType` will take care of that so that it can be applied consistently everywhere.

`ToIdent` is only for code simplification. We decided that we liked the look of `my_string.to_ident()` better than `format_ident!("{}", my_string)`. So that's what we are doing.
