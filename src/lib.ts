import { invoke } from "@tauri-apps/api/core";

export async function handleNewNote(event: Event) {
  event.preventDefault();
  const form = event.target as HTMLFormElement;
  const input = form.querySelector('#new-note-input') as HTMLInputElement;
  const noteText = input.value;

  if (noteText.trim()) {
    try {
      // TODO: Add Rust command to save note
      alert('Note saved');
      input.value = ''; // Clear input after submission
    } catch (error) {
      console.error('Error saving note:', error);
    }
  }
}
