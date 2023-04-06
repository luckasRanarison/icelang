-- icelang syntaxes 🥶
-- Designed to be minimal and concise, inspired by lua and rust

-- inline comment

<--
    This 
    is 
    a 
    multi
    line
    comment 
-->

-- module system
expose "module_name"; -- expose all exported module variables, functions 

set default_import = import("module_name"); -- default import
set component = import("module_name.component"); -- import a specific module component
set component_with_path = import("../module_name.component"); -- unix like path

-- data types 
set string = "Hello World";
set number = 123;
set float = 1.2;
set boolean = true;
set array = [1, 2, 3, 4];
set undefined; -- null

-- constant
freeze constant = "constant value"; 

-- function
function hello(name) {
    print("Hello " + name);
}

-- loops
-- explicit step
for(set i = 1 to 5, i+= 1) {
    print("Hello World");
}

-- non explicit, default is 1 or -1 if downwards
for(set i = 5 to 1) {
    if (i == 3) {
        continue;
    }

    print("Hello World");
}

set i = 1;

while(i <= 5) {
    print("Hello World");
    i += 1;
}

set i = 1;

loop {
    i = i + 1;

    if (i >= 5) {
        break;
    }
}

-- collection loop
foreach(item in arr) {
    print(item);
}

set cond_1 = true;
set cond_2 = false;

-- conditionals
if (cond_1 and cond_2) {
    print("no");
} else if (cond_1) {
    print("yes");
} else {
    print("yes");
}

set a = 1;

match(a) {
    1: a = a + 2;
    2: {
        print("unreachable");    
    }
    _: {
        print("Default");
    }
}

-- module export
set my_var = "some text";
set another_var = "yet another text";
set yet_another_var = "and another one";

-- only one export/export_default statement per file is allowed
-- default export
export_default(my_var); 

-- multiple export
export(another_var, yet_another_var);