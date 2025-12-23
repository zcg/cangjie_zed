; Zed text objects for Cangjie
; Supports: @class, @function, @comment (inside/around)

; Classes and similar constructs
(classDefinition
  (classBody) @class.inside
) @class.around

(structDefinition
  (structBody) @class.inside
) @class.around

(interfaceDefinition
  (interfaceBody) @class.inside
) @class.around

(enumDefinition
  (enumBody) @class.inside
) @class.around

; Functions
(functionDefinition
  (block) @function.inside
) @function.around

(operatorFunctionDefinition
  (block) @function.inside
) @function.around

(mainDefinition
  (block) @function.inside
) @function.around

; Comments
(lineComment) @comment.inside @comment.around

(blockComment) @comment.inside @comment.around
