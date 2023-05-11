# Stakeholder Review

This document is addressing [PR#9](https://github.com/WebAssembly/wasi-messaging/pull/9), which focused on aligning the interface according to messaging stakeholders.

The review suggested creating examples for a variety of use cases to, in practice, catch where the interface falls short. These were the requested examples:
1. streamed events with checkpointing with a single consumer
1. streamed events with checkpointing with a concurrent consumers (spread load across the same consumer group)
1. queued messages with auto-completion (settle depending on callback outcome)
1. queued messages with explicit completion (app decides to settle messages based on content or work outcome)
1. queued messages with group receives
1. message TTL
1. message scheduling
1. message partition-key 
1. custom message metadata
1. batched send
1. batched receive
1. end-to-end flow of binary data (Protobuf) with content-type and schema hints
1. ad-hoc, volatile subscription creation (MQTT & AMQP)

... each of these requests will be addressed in the following sections, where we'll review whether SpiderLightning's implementors have native support of the feature or not.

## Streamed events with checkpointing with a single consumer

| Implementor | Native support |
| ----------- | -------------- |
| NATS        | No             |
| Mosquitto   | No             |
| Kafka       | Yes            |
| Azure S. Bus| No             |

Even though some implementors do not provide native support for checkpointing, it is possible to implement it by storing some message metadata to persistent storage (see stakeholder-review-code-examples/e0-single-consumer-checkpointing for an actual example).

## Streamed events with checkpointing with a concurrent consumers (spread load across the same consumer group)

| Implementor | Native support |
| ----------- | -------------- |
| NATS        | No             |
| Mosquitto   | No             |
| Kafka       | Yes            |
| Azure S. Bus| No             |

As seen before, no implementor (other than Kafka) offers native support for checkpointing, but, in addition, no implementor (again, other than Kafka) offers support for consumer groups either.

However, it is possible to emulate consumer groups with things like: subject based message queuing.
