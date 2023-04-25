export const section = [
  {
    header: "Syntax",
    paragraph:
      "If you have already done lua before icelang should look really familiar to you.",
    code: `-- inline comment
-- some conventions:
-- filename ends with .ic
-- semicolon ';' is optional
-- both ' and " can be used to create strings
-- "set" keyword for defining variables
-- sanke case for identifiers`,
  },
  {
    header: "Data types",
    paragraph: "Icelang provides basic data types and is dynamically typed.",
    code: `set string = "Hello World";
set integer = 123;
set float = 1.2;
set bool = true;
set range = 0 to 5; -- non inclusive 
set array = [1, 2, 3, 4];
set undefined = null;
set object = {
    prop: "value",
    another: 1,
    method: lambda() {
        print(self.prop); 
    }
}`,
  },
  {
    header: "Operators",
    paragraph: "Here are all icelang operators:",
    code: `-- arithmetic
+ - * / % += -= *= /= %=
-- logic
! == != > < >= <= and or
-- assignments
variable = "value"
array[0] = "value" -- array indexing
object.prop = "value" -- object literal indexing
object[variable] = "value" -- dynamic object indexing`,
  },
  {
    header: "Conditionals",
    paragraph:
      "Conditionals in icelang are expression which means they can return values.",
    code: `set n = if (true) { 1 } else { 0 }
set value = match(n) {
    0, 4, 1: n + 2,
    2: {
        -- block statement
        print("unreachable");
    },
    _: print("Default") ,
} `,
  },
  {
    header: "Loops",
    paragraph: "Icelang provides three types of loop statements.",
    code: `for i in 0 to 5 {
    print("Hello World");
}

-- collection loop: number, string, array and object
for key, value in object {
    print(key + ": " + value);
}

set i = 1;
while(i <= 5) {
    print("Hello World");
    i += 1;
}

set j = 1;
loop {
    j += 1;
    if (j >= 5) {
        break;
    }
}`,
  },
  {
    header: "Functions",
    paragraph: "Icelang provides two ways to declare functions.",
    code: `-- function statement (hoisted)
function hello(name) {
    print("Hello " + name);
}

-- lambda function
set lambda_function = lambda(name) {
    print("Hello " + name);
}`,
  },
  {
    header: "Module system",
    paragraph: "A simple module system for spliting or reusing code.",
    code: `-- module import
set default_import = import("module_name"); -- default import
set component = import("module_name").prop; -- import a specific object prop
set component_with_path = import("../module_name"); -- unix like path

-- module export
set my_var = "some text";
-- only one export per file is allowed
export(my_var);
    `,
  },
  {
    header: "Builtins",
    paragraph: "Some builtin functions:",
    code: `-- I/O
print("Hello World");
set input = readline();

-- Utility
type_of(a);
parse_number("1");
length("hello");

-- Math
sqrt(8);
pow(2, 5);
floor(2.5);
round(2.5);
ceil(2.5);`,
  },
];
