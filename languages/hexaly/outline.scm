(function_declaration
  "function" @context
  name: (identifier) @name) @item

(class_declaration
  "class" @context
  name: (identifier) @name) @item

(method_declaration
  name: (identifier) @name) @item

(constructor_declaration
  "constructor" @name) @item

; Model declarations are the structure of a Hexaly file, so decisions and expression trees
; belong in the outline alongside functions.
(indexed_declaration
  name: (identifier) @name) @item

(objective_statement
  direction: _ @name) @item
