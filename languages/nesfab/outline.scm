(struct_definition
    "struct" @context
    (identifier) @name) @item

(vars_definition
    "vars" @context
    (group_identifier) @name) @item

(function_definition
    "fn" @context
    function_name: (identifier) @name) @item
