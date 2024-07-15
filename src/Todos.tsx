import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { invoke } from "@tauri-apps/api/core";
import { motion } from "framer-motion";
import { FormEvent, useEffect, useState } from "react";

interface Todo {
  id: string;
  content: string;
}

interface Todos {
  todos: Todo[];
}

export default function TodosPage() {
  const [input, setInput] = useState("");
  const [todos, setTodos] = useState<Todos>({ todos: [] });

  useEffect(() => {
    const fetchTodos = async () => {
      try {
        const todos: Todos = (await invoke("get_todos", {})) as Todos;
        setTodos(todos);
      } catch (error) {
        console.error("Failed to fetch todos", error);
      }
    };

    fetchTodos();
  }, []);

  async function createTodo(e: FormEvent<HTMLFormElement>) {
    e.preventDefault();
    setTodos(await invoke("create_todo", { content: input }));
  }

  return (
    <motion.main
      initial={{ opacity: 0 }}
      animate={{ opacity: 1 }}
      exit={{ opacity: 0 }}
      className="flex flex-1 flex-col gap-4 p-4 lg:gap-6 lg:p-6"
    >
      <div className="flex items-center">
        <h1 className="text-lg font-semibold md:text-2xl">Todos</h1>
      </div>
      <div
        className="flex flex-1 items-center justify-center rounded-lg border border-dashed shadow-sm"
        x-chunk="dashboard-02-chunk-1"
      >
        <div>
          <form onSubmit={createTodo}>
            <Input
              type="text"
              placeholder="Todo"
              value={input}
              onChange={(e) => setInput(e.target.value)}
            />
            <Button type="submit">Enviar</Button>
          </form>
          <ul>
            {todos.todos.map((todo) => (
              <li key={todo.id}>{todo.content}</li>
            ))}
          </ul>
        </div>
      </div>
    </motion.main>
  );
}
