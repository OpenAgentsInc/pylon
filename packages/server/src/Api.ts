import { HttpApiBuilder } from "@effect/platform"
import { TodosApi, TodoId } from "@openagentsinc/domain/TodosApi"
import { Effect, Layer } from "effect"
import { TodosRepository } from "./TodosRepository.js"

const TodosApiLive = HttpApiBuilder.group(TodosApi, "todos", (handlers) =>
  Effect.gen(function*() {
    const todos = yield* TodosRepository
    return handlers
      .handle("getAllTodos", () => todos.getAll)
      .handle("getTodoById", ({ path: { id } }) => todos.getById(TodoId.make(id)))
      .handle("createTodo", ({ payload: { text } }) => todos.create(text))
      .handle("completeTodo", ({ path: { id } }) => todos.complete(TodoId.make(id)))
      .handle("removeTodo", ({ path: { id } }) => todos.remove(TodoId.make(id)))
  }))

export const ApiLive = HttpApiBuilder.api(TodosApi).pipe(
  Layer.provide(TodosApiLive)
)
