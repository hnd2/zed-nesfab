(struct_definition
    "struct" @context
    (identifier) @name) @item

(vars_definition
    "vars" @context
    (group_identifier) @name) @item

(vars_definition
    "vars" @context) @item

(data_definition
    "data" @context
    (group_identifier) @name) @item

(function_definition
    (function_keyword) @context
    (identifier) @name) @item

(asm_function_definition
    (asm_function_keyword) @context
    (identifier) @name) @item

(chrrom_definition
    "chrrom" @context) @item
