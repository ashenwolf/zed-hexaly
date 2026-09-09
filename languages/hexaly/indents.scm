(block "}" @end) @indent
(class_body "}" @end) @indent
(map_literal "}" @end) @indent
(argument_list ")" @end) @indent
(parameter_list ")" @end) @indent
(parenthesized_expression ")" @end) @indent

; Continuation lines of a multi-line declaration, which Hexaly models use heavily for
; index-space declarations broken across lines.
(indexed_declaration) @indent
(declarator) @indent
(constraint_statement) @indent
(objective_statement) @indent
