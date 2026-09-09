(function_declaration
  body: (_
    "{"
    (_)* @function.inside
    "}")) @function.around

(method_declaration
  body: (_
    "{"
    (_)* @function.inside
    "}")) @function.around

(constructor_declaration
  body: (_
    "{"
    (_)* @function.inside
    "}")) @function.around

(lambda) @function.around

(class_declaration
  body: (_
    "{"
    (_)* @class.inside
    "}")) @class.around

(comment) @comment.around
