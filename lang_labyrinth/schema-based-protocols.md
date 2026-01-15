# Interface Definition Languages, (IDLs)
> Schema based protocols, I guess

## Buzzwords
- Schema-driven
- Code-generated
- Language-neutral/agnostic
- Serialisation library

## Pros
- Single source of truth
  - Bindings for many languages from one source
  - Documentation & implementation stay in sync
- More future-proof than hand-rolled solutions
  - Backwards compatibility (or not if you need to get rid of it)
  - Safe deprecation
  - Message evolution
  - New-old interactions (e.g. between client on v0.9 and server on v1.1)
- Language agnostic

## Cons
- Extra dependency
- "uh-uh it's not real programming because it's not hard enough"

## Semantics
- Publisher-subscriber (pub-sub)
- Remote Procedure Call (RPC)
- Request-reply

## Frameworks (IDLs)
- Flatbuffers
  - `flatcc`
- Protobuffers
  - `nanopb` for embedded devices
  - `protobuf-c` too
- Cap'n Proto
  - RPC framework
- JSON Schema
  - Pretty dang common in web methinks
- MessagePack
  - IOT sorta deal??

### Standardised
- ASN.1 IDL
- RFC8939 Concise Binary Object Representation (CBOR)
  - <https://cbor.io>
- RFC8610 Concise Data Definition Language (CDDL)

### Wrapper generators
- Simplified Wrapper and Interface Generator (SWIG)
  - <https://www.swig.org>

### Other
- FIDL
  - From Google's Fuschia OS
- Zerio
- Bond
  - From Microsoft

### Serialisation formats
- Python's `pickle` faculty
