use crate::typechecker::{Type, TypeResult};

pub struct TypeCompatibility;

impl TypeCompatibility {
    /// Refine a type based on type annotation information
    pub fn refine_type_with_annotation(inferred: &Type, annotated: &Type) -> TypeResult<Type> {
        match (inferred, annotated) {
            // If the inferred type has Unknown components, use the annotated type
            (
                Type::Function {
                    param: inf_param,
                    result: inf_result,
                },
                Type::Function {
                    param: ann_param,
                    result: ann_result,
                },
            ) => {
                // If inferred parameter is Unknown, use annotated parameter type
                let refined_param = if matches!(**inf_param, Type::Unknown) {
                    ann_param.clone()
                } else {
                    inf_param.clone()
                };

                // If inferred result is Unknown, use annotated result type
                let refined_result = if matches!(**inf_result, Type::Unknown) {
                    ann_result.clone()
                } else {
                    inf_result.clone()
                };

                Ok(Type::Function {
                    param: refined_param,
                    result: refined_result,
                })
            }
            (Type::List { element: inf_elem }, Type::List { element: ann_elem }) => {
                // If inferred element is Unknown, use annotated element type
                let refined_element = if matches!(**inf_elem, Type::Unknown) {
                    ann_elem.clone()
                } else {
                    inf_elem.clone()
                };

                Ok(Type::List {
                    element: refined_element,
                })
            }
            // Handle sum types with Unknown components
            (
                Type::Sum {
                    left: inf_left,
                    right: inf_right,
                },
                Type::Sum {
                    left: ann_left,
                    right: ann_right,
                },
            ) => {
                let refined_left = if matches!(**inf_left, Type::Unknown) {
                    ann_left.clone()
                } else {
                    inf_left.clone()
                };

                let refined_right = if matches!(**inf_right, Type::Unknown) {
                    ann_right.clone()
                } else {
                    inf_right.clone()
                };

                Ok(Type::Sum {
                    left: refined_left,
                    right: refined_right,
                })
            }
            // For non-function/non-list types, return the inferred type as-is
            _ => Ok(inferred.clone()),
        }
    }

    /// Refine a type by replacing Unknown with more specific types based on context
    pub fn refine_type_with_context(original: &Type, context: &Type) -> Type {
        match (original, context) {
            (Type::Unknown, concrete_type) if !matches!(concrete_type, Type::Unknown) => {
                concrete_type.clone()
            }
            // Handle List types with Unknown elements
            (
                Type::List { element },
                Type::List {
                    element: context_element,
                },
            ) => Type::List {
                element: Box::new(Self::refine_type_with_context(element, context_element)),
            },
            // If original is Unknown but context suggests List, use List with Unknown elements
            (Type::Unknown, Type::List { .. }) => context.clone(),
            // If context has Unknown but original is more specific, prefer original
            (original_type, Type::Unknown) => original_type.clone(),
            // Handle Sum types with Unknown components
            (
                Type::Sum { left, right },
                Type::Sum {
                    left: context_left,
                    right: context_right,
                },
            ) => Type::Sum {
                left: Box::new(Self::refine_type_with_context(left, context_left)),
                right: Box::new(Self::refine_type_with_context(right, context_right)),
            },
            // If original is Unknown but context suggests Sum, use Sum
            (Type::Unknown, Type::Sum { .. }) => context.clone(),
            (Type::Function { param, result }, _) => Type::Function {
                param: Box::new(Self::refine_type_with_context(param, &Type::Unknown)),
                result: Box::new(Self::refine_type_with_context(result, &Type::Unknown)),
            },
            _ => original.clone(),
        }
    }

    /// Check if two types are compatible
    pub fn types_compatible(t1: &Type, t2: &Type) -> bool {
        match (t1, t2) {
            // Unknown types are compatible with anything
            (Type::Unknown, _) | (_, Type::Unknown) => true,

            // Function types are compatible if their parameters and results are compatible
            (
                Type::Function {
                    param: p1,
                    result: r1,
                },
                Type::Function {
                    param: p2,
                    result: r2,
                },
            ) => Self::types_compatible(p1, p2) && Self::types_compatible(r1, r2),

            // List types are compatible if their element types are compatible
            (Type::List { element: e1 }, Type::List { element: e2 }) => {
                Self::types_compatible(e1, e2)
            }

            // Pair types are compatible if their first and second types are compatible
            (
                Type::Pair {
                    first: f1,
                    second: s1,
                },
                Type::Pair {
                    first: f2,
                    second: s2,
                },
            ) => Self::types_compatible(f1, f2) && Self::types_compatible(s1, s2),

            // Sum types are compatible if their left and right types are compatible
            (
                Type::Sum {
                    left: l1,
                    right: r1,
                },
                Type::Sum {
                    left: l2,
                    right: r2,
                },
            ) => Self::types_compatible(l1, l2) && Self::types_compatible(r1, r2),

            // Otherwise, use structural equality
            _ => t1 == t2,
        }
    }
}
