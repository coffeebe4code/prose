trait Car = {
  make: string,
  model: string,
  drive: (self, direction: string) => void
}

// must implement drive
type Buick: Car = {
  drive: (self, direction: string) => void = {
    print("make {}: is driving {}", self.make, direction);

  } 
}

// notice how Buick does not implement make or model. 
// In rust one could just make `make` and `model` methods, and force `Buick` to implement them.
// This is just a shorter end result.
// Afterall, with traits, there is nothing saying how Car must be implemented. Those are two bonus properties that Buick Could work with.

fn main: () => void = {
  let buick = Buick {};

}


// much like rust there is no formal constructor method. It is easy to implement your own.

type Buick: Car = {
  pub new: (model: string) => Self = {
    return Buick { "Buick", model };
  }
  drive: (self, direction: string) => void = {
    print("make {}: is driving {}", self.make, direction);

  } 
}

fn main: () => void {
  let buick = Buick::new("LeSabre");
}

// move, read, and copy semantics, or just (move semantics)
// in order to prevent the need for a garbage collector, there needs to be special rules.
// every scalar value is copied, always.

let x = 5;
let y = x;
y = 5 - y;

// anything that fits in a register is copied. it would be rediculous if y pointed to x, so that y getting assigned 5 - y, would also set x to 0.
// anything larger than a register, gets moved.

let obj = { x = 5, y = 3 };
let moved = obj;
obj.x = 7; // <= UH OH

// this is not allowed

// self is special it owns the object, so it can freely update, call, and read from itself.
// from earlier example with drive.

type Buick: Car = {
  pub new: (model: string) => Self = {
    return Buick { "Buick", model };
  }
  drive: (self, direction: string) => void = {
    print("make {}: is driving {}", self.make, direction);

  } 
}

fn main: () => void {
  let buick = Buick::new("LeSabre");
  buick.drive("north");
}
// buick is not moved into the drive function.

// prose does not call this the same as rust ownership and borrow checking. 
// prose is much more relaxed, and does not work with references.
// this is why direction works from earlier example.

// this would not work.

fn add_world: (hello: string) => void = {
  hello = hello + "world"
}

// hello is now moved
// in order to do what we need and take advantage of move semantics we must return the new result.

fn add_world: (hello: string) => string = {
  hello = hello + "world";
  return hello
}

fn main: () => void {
  let hello = "hello";
  let hello_world = add_world(hello);
}

// hello has been moved! it is no longer referenceable
// we can clean up some readability too

fn main: () => void {
  let hello = "hello";
  hello = add_world(hello);
}

// we can do even more

fn main: () => void {
  let hello = add_world("hello");
}

// sometimes we may desire to copy the object. This has performance implications, but could be something we desire.

let obj = { x = 5, y = 3 };
let duplicate = copy obj;
duplicate.x = 7;
obj.x = 2;

// if we were not reassigning the string from earlier, we could copy.

fn add_world: (hello: string) => string = {
  let new_string = copy hello + "world";
  return new_string;
}







