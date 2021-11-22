// Copyright (C) 2019-2021 Aleo Systems Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

use crate::*;
use indexmap::IndexMap;
use leo_errors::{Result, Span};

// Needed to fix clippy bug.
#[allow(clippy::redundant_closure)]
pub trait ReconstructingReducer {
    fn in_struct(&self) -> bool;
    fn swap_in_struct(&mut self);

    fn reduce_type(&mut self, _type_: &Type, new: Type, _span: &Span) -> Result<Type> {
        Ok(new)
    }

    // Expressions
    fn reduce_expression(&mut self, _expression: &Expression, new: Expression) -> Result<Expression> {
        Ok(new)
    }

    fn reduce_identifier(&mut self, identifier: &Identifier) -> Result<Identifier> {
        Ok(Identifier {
            name: identifier.name.clone(),
            span: identifier.span.clone(),
        })
    }

    fn reduce_group_tuple(&mut self, group_tuple: &GroupTuple) -> Result<GroupTuple> {
        Ok(GroupTuple {
            x: group_tuple.x.clone(),
            y: group_tuple.y.clone(),
            span: group_tuple.span.clone(),
        })
    }

    fn reduce_group_value(&mut self, _group_value: &GroupValue, new: GroupValue) -> Result<GroupValue> {
        Ok(new)
    }

    fn reduce_string(&mut self, string: &[Char], span: &Span) -> Result<Expression> {
        Ok(Expression::Value(ValueExpression::String(
            string.to_vec(),
            span.clone(),
        )))
    }

    fn reduce_value(&mut self, _value: &ValueExpression, new: Expression) -> Result<Expression> {
        Ok(new)
    }

    fn reduce_binary(
        &mut self,
        binary: &BinaryExpression,
        left: Expression,
        right: Expression,
        op: BinaryOperation,
    ) -> Result<BinaryExpression> {
        Ok(BinaryExpression {
            left: Box::new(left),
            right: Box::new(right),
            op,
            span: binary.span.clone(),
        })
    }

    fn reduce_unary(
        &mut self,
        unary: &UnaryExpression,
        inner: Expression,
        op: UnaryOperation,
    ) -> Result<UnaryExpression> {
        Ok(UnaryExpression {
            inner: Box::new(inner),
            op,
            span: unary.span.clone(),
        })
    }

    fn reduce_ternary(
        &mut self,
        ternary: &TernaryExpression,
        condition: Expression,
        if_true: Expression,
        if_false: Expression,
    ) -> Result<TernaryExpression> {
        Ok(TernaryExpression {
            condition: Box::new(condition),
            if_true: Box::new(if_true),
            if_false: Box::new(if_false),
            span: ternary.span.clone(),
        })
    }

    fn reduce_cast(&mut self, cast: &CastExpression, inner: Expression, target_type: Type) -> Result<CastExpression> {
        Ok(CastExpression {
            inner: Box::new(inner),
            target_type,
            span: cast.span.clone(),
        })
    }

    fn reduce_array_access(
        &mut self,
        array_access: &ArrayAccess,
        array: Expression,
        index: Expression,
    ) -> Result<ArrayAccess> {
        Ok(ArrayAccess {
            array: Box::new(array),
            index: Box::new(index),
            span: array_access.span.clone(),
        })
    }

    fn reduce_array_range_access(
        &mut self,
        array_rage_access: &ArrayRangeAccess,
        array: Expression,
        left: Option<Expression>,
        right: Option<Expression>,
    ) -> Result<ArrayRangeAccess> {
        Ok(ArrayRangeAccess {
            array: Box::new(array),
            left: left.map(|expr| Box::new(expr)),
            right: right.map(|expr| Box::new(expr)),
            span: array_rage_access.span.clone(),
        })
    }

    fn reduce_member_access(
        &mut self,
        member_access: &MemberAccess,
        inner: Expression,
        name: Identifier,
        type_: Option<Type>,
    ) -> Result<MemberAccess> {
        Ok(MemberAccess {
            inner: Box::new(inner),
            name,
            span: member_access.span.clone(),
            type_,
        })
    }

    fn reduce_tuple_access(&mut self, tuple_access: &TupleAccess, tuple: Expression) -> Result<TupleAccess> {
        Ok(TupleAccess {
            tuple: Box::new(tuple),
            index: tuple_access.index.clone(),
            span: tuple_access.span.clone(),
        })
    }

    fn reduce_static_access(
        &mut self,
        static_access: &StaticAccess,
        value: Expression,
        type_: Option<Type>,
        name: Identifier,
    ) -> Result<StaticAccess> {
        Ok(StaticAccess {
            inner: Box::new(value),
            name,
            type_,
            span: static_access.span.clone(),
        })
    }

    fn reduce_array_inline(
        &mut self,
        array_inline: &ArrayInlineExpression,
        elements: Vec<SpreadOrExpression>,
    ) -> Result<ArrayInlineExpression> {
        Ok(ArrayInlineExpression {
            elements,
            span: array_inline.span.clone(),
        })
    }

    fn reduce_array_init(
        &mut self,
        array_init: &ArrayInitExpression,
        element: Expression,
    ) -> Result<ArrayInitExpression> {
        Ok(ArrayInitExpression {
            element: Box::new(element),
            dimensions: array_init.dimensions.clone(),
            span: array_init.span.clone(),
        })
    }

    fn reduce_tuple_init(
        &mut self,
        tuple_init: &TupleInitExpression,
        elements: Vec<Expression>,
    ) -> Result<TupleInitExpression> {
        Ok(TupleInitExpression {
            elements,
            span: tuple_init.span.clone(),
        })
    }

    fn reduce_struct_implied_variable_definition(
        &mut self,
        _variable: &StructImpliedVariableDefinition,
        identifier: Identifier,
        expression: Option<Expression>,
    ) -> Result<StructImpliedVariableDefinition> {
        Ok(StructImpliedVariableDefinition { identifier, expression })
    }

    fn reduce_struct_init(
        &mut self,
        struct_init: &StructInitExpression,
        name: Identifier,
        members: Vec<StructImpliedVariableDefinition>,
    ) -> Result<StructInitExpression> {
        Ok(StructInitExpression {
            name,
            members,
            span: struct_init.span.clone(),
        })
    }

    fn reduce_call(
        &mut self,
        call: &CallExpression,
        function: Expression,
        arguments: Vec<Expression>,
    ) -> Result<CallExpression> {
        Ok(CallExpression {
            function: Box::new(function),
            arguments,
            span: call.span.clone(),
        })
    }

    // Statements
    fn reduce_statement(&mut self, _statement: &Statement, new: Statement) -> Result<Statement> {
        Ok(new)
    }

    fn reduce_return(&mut self, return_statement: &ReturnStatement, expression: Expression) -> Result<ReturnStatement> {
        Ok(ReturnStatement {
            expression,
            span: return_statement.span.clone(),
        })
    }

    fn reduce_variable_name(&mut self, variable_name: &VariableName, identifier: Identifier) -> Result<VariableName> {
        Ok(VariableName {
            mutable: variable_name.mutable,
            identifier,
            span: variable_name.span.clone(),
        })
    }

    fn reduce_definition(
        &mut self,
        definition: &DefinitionStatement,
        variable_names: Vec<VariableName>,
        type_: Option<Type>,
        value: Expression,
    ) -> Result<DefinitionStatement> {
        Ok(DefinitionStatement {
            declaration_type: definition.declaration_type.clone(),
            variable_names,
            type_,
            value,
            span: definition.span.clone(),
        })
    }

    fn reduce_assignee_access(&mut self, _access: &AssigneeAccess, new: AssigneeAccess) -> Result<AssigneeAccess> {
        Ok(new)
    }

    fn reduce_assignee(
        &mut self,
        assignee: &Assignee,
        identifier: Identifier,
        accesses: Vec<AssigneeAccess>,
    ) -> Result<Assignee> {
        Ok(Assignee {
            identifier,
            accesses,
            span: assignee.span.clone(),
        })
    }

    fn reduce_assign(
        &mut self,
        assign: &AssignStatement,
        assignee: Assignee,
        value: Expression,
    ) -> Result<AssignStatement> {
        Ok(AssignStatement {
            operation: assign.operation,
            assignee,
            value,
            span: assign.span.clone(),
        })
    }

    fn reduce_conditional(
        &mut self,
        conditional: &ConditionalStatement,
        condition: Expression,
        block: Block,
        statement: Option<Statement>,
    ) -> Result<ConditionalStatement> {
        Ok(ConditionalStatement {
            condition,
            block,
            next: statement.map(|statement| Box::new(statement)),
            span: conditional.span.clone(),
        })
    }

    fn reduce_iteration(
        &mut self,
        iteration: &IterationStatement,
        variable: Identifier,
        start: Expression,
        stop: Expression,
        block: Block,
    ) -> Result<IterationStatement> {
        Ok(IterationStatement {
            variable,
            start,
            stop,
            inclusive: iteration.inclusive,
            block,
            span: iteration.span.clone(),
        })
    }

    fn reduce_console(&mut self, console: &ConsoleStatement, function: ConsoleFunction) -> Result<ConsoleStatement> {
        Ok(ConsoleStatement {
            function,
            span: console.span.clone(),
        })
    }

    fn reduce_expression_statement(
        &mut self,
        expression_statement: &ExpressionStatement,
        expression: Expression,
    ) -> Result<ExpressionStatement> {
        Ok(ExpressionStatement {
            expression,
            span: expression_statement.span.clone(),
        })
    }

    fn reduce_block(&mut self, block: &Block, statements: Vec<Statement>) -> Result<Block> {
        Ok(Block {
            statements,
            span: block.span.clone(),
        })
    }

    #[allow(clippy::too_many_arguments)]
    // Program
    fn reduce_program(
        &mut self,
        program: &Program,
        expected_input: Vec<FunctionInput>,
        import_statements: Vec<ImportStatement>,
        imports: IndexMap<Vec<String>, Program>,
        aliases: IndexMap<Identifier, Alias>,
        structs: IndexMap<Identifier, Struct>,
        functions: IndexMap<Identifier, Function>,
        global_consts: IndexMap<Vec<Identifier>, DefinitionStatement>,
    ) -> Result<Program> {
        Ok(Program {
            name: program.name.clone(),
            expected_input,
            import_statements,
            imports,
            aliases,
            structs,
            functions,
            global_consts,
        })
    }

    fn reduce_function_input_variable(
        &mut self,
        variable: &FunctionInputVariable,
        identifier: Identifier,
        type_: Type,
    ) -> Result<FunctionInputVariable> {
        Ok(FunctionInputVariable {
            identifier,
            const_: variable.const_,
            mutable: variable.mutable,
            type_,
            span: variable.span.clone(),
        })
    }

    fn reduce_function_input(&mut self, _input: &FunctionInput, new: FunctionInput) -> Result<FunctionInput> {
        Ok(new)
    }

    fn reduce_package_or_packages(
        &mut self,
        _package_or_packages: &PackageOrPackages,
        new: PackageOrPackages,
    ) -> Result<PackageOrPackages> {
        Ok(new)
    }

    fn reduce_import_statement(
        &mut self,
        import: &ImportStatement,
        package_or_packages: PackageOrPackages,
    ) -> Result<ImportStatement> {
        Ok(ImportStatement {
            package_or_packages,
            span: import.span.clone(),
        })
    }

    fn reduce_import(&mut self, identifier: Vec<String>, import: Program) -> Result<(Vec<String>, Program)> {
        Ok((identifier, import))
    }

    fn reduce_struct_member(&mut self, _struct_member: &StructMember, new: StructMember) -> Result<StructMember> {
        Ok(new)
    }

    fn reduce_struct(
        &mut self,
        _structure: &Struct,
        struct_name: Identifier,
        members: Vec<StructMember>,
    ) -> Result<Struct> {
        Ok(Struct { struct_name, members })
    }

    fn reduce_annotation(&mut self, annotation: &Annotation, name: Identifier) -> Result<Annotation> {
        Ok(Annotation {
            span: annotation.span.clone(),
            name,
            arguments: annotation.arguments.clone(),
        })
    }

    #[allow(clippy::too_many_arguments)]
    fn reduce_function(
        &mut self,
        function: &Function,
        identifier: Identifier,
        annotations: IndexMap<String, Annotation>,
        input: Vec<FunctionInput>,
        const_: bool,
        output: Option<Type>,
        block: Block,
    ) -> Result<Function> {
        Ok(Function {
            identifier,
            annotations,
            input,
            const_,
            output,
            block,
            core_mapping: function.core_mapping.clone(),
            span: function.span.clone(),
        })
    }
}
