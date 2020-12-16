
outer:
for (let i = 0; i < 3; ++i) {
    for (let D = 0; D < 10; ++D) {
        console.log("D", D);
        if (8===D) {
            continue outer;
        }
    }

    console.log("Foo");
}

