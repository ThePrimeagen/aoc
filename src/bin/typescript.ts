let value = 0;
class Foo {
	public value: number = ++value;
}

const foo = [new Foo(), new Foo(), new Foo()];

function printsInReverse(foo: Foo[]) {
	foo.reverse().forEach(console.log);
}

printsInReverse(foo);
printsInReverse(foo);


