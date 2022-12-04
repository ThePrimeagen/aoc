
type FooEvent = {
    ack: {ack: string},
    bash: {bash: string},
    bar: {bar: string},
    buzz: {buzz: string},
}

type BarEvent = {
    ack: {ack: string},
    bash: {bash: string},
    buzz: {buzz: string},
}

type TypeToEvent = {
    "foo": FooEvent,
    "bar": BarEvent,
}


function isFish(e: FooEvent | BarEvent): e is FooEvent {
    return "bar" in e;
}

function write<T extends keyof TypeToEvent>(
    type: T, event: T extends "foo" ? FooEvent : BarEvent) {

    if (type === "foo") {
        event.bar
    }
}

write("foo", {
    ack: { ack: "ack"},
    bar: { bar: "bar"},
    bash: {bash: "bash"},
    buzz: {buzz: "buzz"}
});



