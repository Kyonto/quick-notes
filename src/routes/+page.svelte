<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { handleNewNote } from "../lib";
  import { tick } from 'svelte';
  import "../app.css";

  let content = '';
  let title = '';
  let showToast = false;
  let toastMessage = '';
  let tags = '';

  function showToastMessage(message: string) {
    toastMessage = message;
    showToast = true;
    setTimeout(() => {
      showToast = false;
    }, 3000);
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

<main class="m-0 p-0 h-screen w-screen fixed flex flex-col overflow-hidden bg-black">
  <header class="h-[7%] w-full flex justify-center items-center bg-gray-900/90 shadow-md font-mono border-b border-gray-800">
    <h1 class="text-3xl font-bold text-gray-100">Notes</h1>
  </header>
  
  <div class="h-[88%] w-full flex flex-col relative bg-gray-900">
    <div class="notes-container">
    </div>
    <div class="flex justify-center w-[90%] items-center absolute bottom-0 left-1/2 -translate-x-1/2 p-4 bg-gray-900/90 shadow-md border-t border-gray-800">
      <div class="w-full mx-auto flex items-center justify-center flex-col gap-2">
        <input
          type="text"
          class="w-full p-2 border border-gray-700 rounded-lg text-base outline-none transition-colors duration-200 focus:border-blue-500 bg-gray-800 text-gray-100 placeholder-gray-400 font-mono"
          bind:value={title}
          placeholder="Enter title..."
          on:keydown={handleInputAreaKeydown}
        />
        <textarea 
          class="w-full min-h-[80px] max-h-[30vh] p-2 border border-gray-700 rounded-lg text-base resize-vertical outline-none transition-colors duration-200 focus:border-blue-500 bg-gray-800 text-gray-100 placeholder-gray-400 font-mono"
          bind:value={content}
          placeholder="Write something..."
          rows="5"
          on:keydown={handleInputAreaKeydown}
        ></textarea>
        <textarea 
          class="w-full min-h-[20px] max-h-[10vh] p-2 border border-gray-700 rounded-lg text-base resize-vertical outline-none transition-colors duration-200 focus:border-blue-500 bg-gray-800 text-gray-100 placeholder-gray-400 font-mono"
          bind:value={tags}
          placeholder="Tags..."
          rows="1"
          on:keydown={handleInputAreaKeydown}
        ></textarea>
      </div>
    </div>
  </div>

  <footer class="h-[5%] w-full bg-gray-900/90 shadow-md border-t border-gray-800">
  </footer>
</main>

{#if showToast}
  <div class="fixed top-4 right-4 bg-gray-800 text-gray-100 px-5 py-3 rounded-lg shadow-lg z-50 border border-gray-700">
    {toastMessage}
  </div>
{/if}

<style>
</style>
