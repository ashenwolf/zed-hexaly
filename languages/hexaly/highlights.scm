; Zed resolves overlapping captures by pattern order: the LAST matching pattern wins (the
; nvim-treesitter/Helix convention — the opposite of the tree-sitter CLI's first-wins). So
; this file is ordered general-to-specific: the `(identifier) @variable` catch-all comes
; before every specific identifier capture, otherwise it would repaint functions,
; parameters and properties as plain variables.

; Modeling keywords first: these are what distinguish a Hexaly model from a script, so they
; get @keyword and are listed before the general control-flow set.
[
  "constraint"
  "minimize"
  "maximize"
] @keyword

[
  "if"
  "else"
  "for"
  "while"
  "do"
  "in"
  "return"
  "break"
  "continue"
  "try"
  "catch"
  "throw"
] @keyword

[
  "function"
  "local"
  "class"
  "extends"
  "constructor"
  "new"
  "use"
  "from"
  "as"
  "pragma"
  "typeof"
  "is"
  "with"
] @keyword

(modifier) @keyword

; Reserved but unused by the language. Highlighting them as errors makes accidental use
; visible instead of silently parsing as an identifier.
(reserved_identifier) @keyword.unused

[
  (this_expression)
  (super_expression)
] @variable.special

(primitive_type) @type.builtin

(anonymous_function
  "function" @keyword)

; The identifier catch-all. Everything below refines it for specific grammatical roles.
(identifier) @variable

(parameter_list
  parameter: (identifier) @variable.parameter)

; Loop and aggregate index variables read as parameters: they are bound by the range, not
; assigned by the body.
(index_range
  index: (identifier) @variable.parameter)

(member_expression
  property: (identifier) @property)

(pair
  key: (identifier) @property)

(field_declaration
  name: (identifier) @property)

(module_path
  (identifier) @namespace)

(use_statement
  alias: (identifier) @namespace)

(import_specifier
  name: (identifier) @type)

(import_specifier
  alias: (identifier) @type)

(function_declaration
  name: (identifier) @function)

(method_declaration
  name: (identifier) @function.method)

(call_expression
  function: (identifier) @function)

; Called member properties are methods, so this must come after the plain @property rule.
(call_expression
  function: (member_expression
    property: (identifier) @function.method))

; Builtins refine the generic call captures above.
(call_expression
  function: (identifier) @function.builtin
  (#any-of? @function.builtin
    "abs" "and" "array" "at" "bool" "call" "ceil" "contains" "cos" "count" "cover" "disjoint"
    "dist" "div" "doubleArrayExternalFunction" "doubleExternalFunction" "end" "eq" "exp" "find"
    "float" "floor" "geq" "gt" "iif" "int" "intArrayExternalFunction" "intExternalFunction"
    "indexOf" "interval" "length" "leq" "list" "log" "lt" "max" "min" "mod" "neq" "not"
    "partition" "piecewise" "pow" "prod" "round" "scalar" "set" "sin" "sort" "sqrt" "start"
    "sub" "sum" "tan" "xor"))

; The official variadic-call aggregates. `count` is deliberately absent: it takes a single
; collection argument and is not a variadic aggregate. `argmin`/`argmax` are not documented.
(aggregate_expression
  operator: (identifier) @function.builtin
  (#any-of? @function.builtin "sum" "prod" "min" "max" "and" "or"))

(class_declaration
  name: (identifier) @type)

(class_declaration
  superclass: (identifier) @type)

(new_expression
  class: (identifier) @type)

(string) @string
(escape_sequence) @string.escape
(number) @number
(boolean) @boolean

[
  (nil)
  (float_constant)
] @constant.builtin

(comment) @comment

; Only two `#` forms exist in HXM — a first-line shebang and an encoding declaration.
[
  (shebang)
  (encoding_declaration)
] @preproc

; `<-` binds a decision or expression tree to a name and is the core modeling operator, so
; it is highlighted as a keyword rather than a plain operator.
"<-" @keyword.operator

[
  "="
  "+="
  "-="
  "*="
  "/="
  "%="
  "+"
  "-"
  "*"
  "/"
  "%"
  "=="
  "!="
  "<"
  "<="
  ">"
  ">="
  "&&"
  "||"
  "!"
  ".."
  "..."
  "=>"
  "?"
] @operator

[
  ";"
  ","
  "."
  ":"
] @punctuation.delimiter

[
  "("
  ")"
  "["
  "]"
  "{"
  "}"
] @punctuation.bracket
