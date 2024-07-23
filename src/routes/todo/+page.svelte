<script lang="ts">
    import { onMount } from "svelte";
    import { writable } from "svelte/store";
    import { invoke } from "@tauri-apps/api/core";
    import Database from "@tauri-apps/plugin-sql";
    // import { fetchTodos } from '../lib/database';
    interface Tasks {
        id: number;
        title: string;
        description: string | null;
    }
    let taskName = "";
    let taskDescription = "";
    let addTodoItem = "";
    let todoList: Tasks[] = [];

    export const todos = writable<Tasks[]>([]);

    export async function fetchTodos(): Promise<void> {
        try {
            const db = await Database.load("sqlite:p4todo.db"); // Adjust the connection string as needed
            const result: Tasks[] = await db.select<Tasks[]>("SELECT * FROM tasks");

            // Update the Svelte store with the fetched data
            todos.set(result);
        } catch (error) {
            console.error("Failed to fetch tasks:", error);
        }
    }

    onMount(async () => {
        await fetchTodos();
    });

    $: todoList = $todos;

    async function add_todo() {
        // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
        addTodoItem = await invoke("add_todo", { taskName });
        insert(taskName, taskDescription);
    }

    async function insert(taskName: string, taskDescription: string) {
        const db = await Database.load("sqlite:p4todo.db");
        if (taskName === "" || taskName === null) {
            console.log("empty");
        }
        await db.execute("INSERT INTO tasks (title, description) VALUES ($1, $2)", [taskName, taskDescription]);
        fetchTodos();
    }

    async function selectUsers() {
        const db = await Database.load("sqlite:p4todo.db");
        if (db) {
            const todosSql = await db.select("SELECT * FROM tasks");
            console.log(todosSql);
            //  for
            //  todos = [...todos, todosSql]
        } else {
            console.log("no db?");
        }
    }
 
    selectUsers();
     
    async function deleteMe(id: number) {
        const db = await Database.load("sqlite:p4todo.db");
        const removeItem = await db.select("DELETE FROM tasks WHERE id = $1;", [id]);
        fetchTodos();
    }

</script>


<div>
    <form class="row" on:submit|preventDefault={add_todo}>
        <input
            id="task-input"
            placeholder="Enter a name"
            bind:value={taskName}
        />
        <textarea
            id="task-description"
            placeholder="What have you been busy with?..."
            bind:value={taskDescription}
        />
        <button type="submit">Add</button>
    </form>
    <p>{addTodoItem}</p>
    
    <table>
        <thead>
          <tr>
            <th>ID</th>
            <th>Name</th>
          </tr>
        </thead>
        <tbody>
          {#each todoList as { id, title, description }}
            <tr>
              <td>{id}</td>
              <td>{title}</td>
              <td>{description}</td>
              <td><button on:click={deleteMe(id)}>delete</button></td>
            </tr>
          {/each}
        </tbody>
      </table>
</div>

