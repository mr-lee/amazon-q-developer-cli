// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListTasksInput {
    #[allow(missing_docs)] // documentation missing in model
    pub max_results: ::std::option::Option<i32>,
    #[allow(missing_docs)] // documentation missing in model
    pub next_token: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub task_filters: ::std::option::Option<::std::vec::Vec<crate::types::TaskFilter>>,
}
impl ListTasksInput {
    #[allow(missing_docs)] // documentation missing in model
    pub fn max_results(&self) -> ::std::option::Option<i32> {
        self.max_results
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }

    #[allow(missing_docs)] // documentation missing in model
    /// If no value was sent for this field, a default will be set. If you want to determine if no
    /// value was sent, use `.task_filters.is_none()`.
    pub fn task_filters(&self) -> &[crate::types::TaskFilter] {
        self.task_filters.as_deref().unwrap_or_default()
    }
}
impl ListTasksInput {
    /// Creates a new builder-style object to manufacture
    /// [`ListTasksInput`](crate::operation::list_tasks::ListTasksInput).
    pub fn builder() -> crate::operation::list_tasks::builders::ListTasksInputBuilder {
        crate::operation::list_tasks::builders::ListTasksInputBuilder::default()
    }
}

/// A builder for [`ListTasksInput`](crate::operation::list_tasks::ListTasksInput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct ListTasksInputBuilder {
    pub(crate) max_results: ::std::option::Option<i32>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    pub(crate) task_filters: ::std::option::Option<::std::vec::Vec<crate::types::TaskFilter>>,
}
impl ListTasksInputBuilder {
    #[allow(missing_docs)] // documentation missing in model
    pub fn max_results(mut self, input: i32) -> Self {
        self.max_results = ::std::option::Option::Some(input);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_results = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        &self.max_results
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        &self.next_token
    }

    /// Appends an item to `task_filters`.
    ///
    /// To override the contents of this collection use
    /// [`set_task_filters`](Self::set_task_filters).
    pub fn task_filters(mut self, input: crate::types::TaskFilter) -> Self {
        let mut v = self.task_filters.unwrap_or_default();
        v.push(input);
        self.task_filters = ::std::option::Option::Some(v);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_task_filters(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::TaskFilter>>) -> Self {
        self.task_filters = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_task_filters(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::TaskFilter>> {
        &self.task_filters
    }

    /// Consumes the builder and constructs a
    /// [`ListTasksInput`](crate::operation::list_tasks::ListTasksInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::list_tasks::ListTasksInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::list_tasks::ListTasksInput {
            max_results: self.max_results,
            next_token: self.next_token,
            task_filters: self.task_filters,
        })
    }
}
