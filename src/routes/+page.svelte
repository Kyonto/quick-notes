<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { handleNewNote } from "../lib";
  import { tick } from 'svelte';

  let content = '';
  let title = '';
  let showToast = false;
  let toastMessage = '';

  function showToastMessage(message: string) {
    toastMessage = message;
    showToast = true;
    setTimeout(() => {
      showToast = false;
    }, 2000);
  }

  function handleSave() {
    if (title === '') {
      showToastMessage('Title is required');
      return;
    }

    console.log('Saving content:', content);
    console.log('Title:', title);

    showToastMessage('Contents saved successfully!');
  }

  function handleInputAreaKeydown(event: KeyboardEvent) {
    if (event.ctrlKey && event.key.toLowerCase() === 's') {
      event.preventDefault();
      handleSave();
    }
  }
</script>

<main class="container">
  <header class="header">
    <h1>Notes</h1>
  </header>
  
  <div class="content">
    <div class="notes-container">
      <div class="notes-list">
      </div>
    </div>
    <div class="note-input-container">
      <div class="input-wrapper">
        <input
          type="text"
          class="title-input"
          bind:value={title}
          placeholder="Enter title..."
          on:keydown={handleInputAreaKeydown}
        />
        <textarea 
          class="note-input"
          bind:value={content}
          placeholder="Write something..."
          rows="5"
          on:keydown={handleInputAreaKeydown}
        ></textarea>
      </div>
    </div>
  </div>

  <footer class="footer">
    
  </footer>
</main>

{#if showToast}
  <div class="toast">{toastMessage}</div>
{/if}

<style>
:root {
  font-family: Inter, system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, 'Open Sans', 'Helvetica Neue', sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;
  color: #0f0f0f;
  background-color: #f6f6f6;
}

.container {
  margin: 0;
  padding: 0;
  height: 100vh;
  width: 100vw;
  position: fixed;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.header {
  height: 7%;
  width: 100%;
  display: flex;
  justify-content: center;
  align-items: center;
  background-color: rgba(255, 255, 255, 0.9);
  box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
}

.content {
  height: 88%;
  width: 100%;
  display: flex;
  flex-direction: column;
  position: relative;
}

.footer {
  height: 5%;
  width: 100%;
  background-color: rgba(255, 255, 255, 0.9);
  box-shadow: 0 -2px 10px rgba(0, 0, 0, 0.1);
}

.note-input-container {
  display: flex;
  justify-content: center;
  width: 90%;
  align-items: center;
  position: absolute;
  bottom: 0;
  left: 50%;
  transform: translateX(-50%);
  padding: 1rem;
  background-color: rgba(255, 255, 255, 0.9);
  box-shadow: 0 -2px 10px rgba(0, 0, 0, 0.1);
}

.input-wrapper {
  width: 100%;
  margin: 0 auto;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-direction: column;
  gap: 0.5rem;
}

.title-input {
  width: 100%;
  padding: 0.5rem;
  border: 1px solid #ddd;
  border-radius: 8px;
  font-size: 1rem;
  outline: none;
  transition: border-color 0.2s ease;
}

.title-input:focus {
  border-color: #4a90e2;
}

.note-input {
  justify-content: center;
  align-items: center;
  width: 100%;
  min-height: 80px;
  max-height: 30vh;
  border: 1px solid #ddd;
  border-radius: 8px;
  font-size: 1rem;
  resize: vertical;
  outline: none;
  transition: border-color 0.2s ease;
  padding: 0.5rem;
}

.note-input:focus {
  border-color: #4a90e2;
}

.toast {
  position: fixed;
  top: 1rem;
  right: 1rem;
  background-color: #2f363267;
  color: white;
  padding: 0.75rem 1.25rem;
  border-radius: 0.5rem;
  box-shadow: 0 4px 12px rgba(0,0,0,0.15);
  opacity: 0;
  animation: fadeInOut 2s forwards;
  z-index: 999;
}

@keyframes fadeInOut {
  0% { opacity: 0; transform: translateY(-10px); }
  10% { opacity: 1; transform: translateY(0); }
  90% { opacity: 1; }
  100% { opacity: 0; transform: translateY(-10px); }
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }
  
  .header, .footer, .note-input-container {
    background-color: rgba(47, 47, 47, 0.9);
  }
  
  .note-input, .title-input {
    background-color: #1f1f1f;
    color: #f6f6f6;
    border-color: #444;
  }
  
  .note-input:focus, .title-input:focus {
    border-color: #6ba7e5;
  }
}
</style>
