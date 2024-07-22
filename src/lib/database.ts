import Database from "@tauri-apps/plugin-sql";
import { writable } from 'svelte/store';

export interface User {
  id: number;
  name: string;
}

export const todos = writable<User[]>([]);

export async function fetchTodos(): Promise<void> {
  try {
    const db = await Database.load('sqlite:test.db'); // Adjust the connection string as needed
    const result = await db.select<User>('SELECT * FROM users');

    // Update the Svelte store with the fetched data
    todos.set(result);
  } catch (error) {
    console.error('Failed to fetch todos:', error);
  }
}