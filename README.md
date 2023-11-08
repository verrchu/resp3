# resp3 implementation (attempt) in rust

this project is an attemp to omplement [resp3](https://github.com/antirez/RESP3/blob/master/spec.md) protocol which is used by [redis](https://redis.io/).
it lacks support for "streamed" messages but is otherwise more-or-less complete

I used [nom](https://docs.rs/nom/latest/nom/) as parsing library and [proptest](https://docs.rs/proptest/latest/proptest/) for "rendom" property based testing
