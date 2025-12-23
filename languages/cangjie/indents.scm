; Zed indentation rules for Cangjie
; Uses @indent and @outdent captures

; Class bodies increase indent
(classDefinition
  (classBody) @indent
)

(structDefinition
  (structBody) @indent
)

(interfaceDefinition
  (interfaceBody) @indent
)

(enumDefinition
  (enumBody) @indent
)

; Function bodies increase indent
(functionDefinition
  (block) @indent
)

(operatorFunctionDefinition
  (block) @indent
)

(mainDefinition
  (block) @indent
)

; Control flow
(ifExpression
  (block) @indent
)

(matchExpression
  (matchCase) @indent
)

(forInExpression
  (block) @indent
)

(whileExpression
  (block) @indent
)

(tryExpression
  (block) @indent
)

; Closing brackets decrease indent
[
  "}"
  "]"
  ")"
] @outdent
