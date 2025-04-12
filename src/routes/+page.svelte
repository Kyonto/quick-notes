<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { handleNewNote } from "../lib";
  import { onMount } from 'svelte';
  import "../app.css";

  interface Note {
    id: number;
    title: string;
    content: string;
    tags: string | null;
    created_at: string;
  }

  interface PaginatedNotes {
    notes: Note[];
    total: number;
  }

  let content = '';
  let title = '';
  let showToast = false;
  let toastMessage = '';
  let tags = '';
  let notes: Note[] = [];
  let currentPage = 1;
  let pageSize = 10;
  let totalNotes = 0;
  let isLoading = false;
  let selectedNoteId: number | null = null;
  let notesContainer: HTMLElement;
  let searchInput: HTMLInputElement;
  let hasMoreNotes = true;
  let searchQuery = '';
  let isSearching = false;
  let searchTimeout: ReturnType<typeof setTimeout>;
  let showDeleteConfirm = false;
  let noteToDelete: Note | null = null;

  async function loadNotes(append = false) {
    if (isLoading || (!append && !hasMoreNotes) || isSearching) return;
    
    isLoading = true;
    try {
      const result = await invoke<PaginatedNotes>('get_paginated_notes', { page: currentPage, pageSize });
      if (append) {
        notes = [...notes, ...result.notes];
      } else {
        notes = result.notes;
      }
      totalNotes = result.total;
      hasMoreNotes = notes.length < totalNotes;
    } catch (error) {
      console.error('Error loading notes:', error);
      showToastMessage('Error loading notes');
    }
    isLoading = false;
  }

  async function handleSearch() {
    if (searchTimeout) {
      clearTimeout(searchTimeout);
    }

    searchTimeout = setTimeout(async () => {
      if (searchQuery.trim() === '') {
        isSearching = false;
        currentPage = 1;
        hasMoreNotes = true;
        await loadNotes();
        return;
      }

      isSearching = true;
      isLoading = true;
      try {
        const searchResults = await invoke<Note[]>('search_notes', { query: searchQuery });
        notes = searchResults;
        hasMoreNotes = false;
      } catch (error) {
        console.error('Error searching notes:', error);
        showToastMessage('Error searching notes');
      }
      isLoading = false;
    }, 300);
  }

  function handleScroll(event: Event) {
    const target = event.target as HTMLElement;
    const threshold = 100; // pixels from bottom
    const bottomPosition = target.scrollHeight - target.scrollTop - target.clientHeight;
    
    if (bottomPosition < threshold && !isLoading && hasMoreNotes) {
      currentPage++;
      loadNotes(true);
    }
  }

  function showToastMessage(message: string) {
    toastMessage = message;
    showToast = true;
    setTimeout(() => {
      showToast = false;
    }, 3000);
  }

  function confirmDelete(note: Note, event: MouseEvent | KeyboardEvent | null) {
    if (event) {
      event.stopPropagation(); // Prevent note selection when clicking delete
    }
    noteToDelete = note;
    showDeleteConfirm = true;
  }

  async function handleDelete() {
    if (!noteToDelete) return;

    try {
      await invoke('delete_note', { noteId: noteToDelete.id });
      showToastMessage('Note deleted successfully');
      
      // Clear selection if the deleted note was selected
      if (selectedNoteId === noteToDelete.id) {
        clearSelection();
      }

      // Refresh the notes list
      if (isSearching && searchQuery.trim() !== '') {
        await handleSearch();
      } else {
        currentPage = 1;
        hasMoreNotes = true;
        await loadNotes();
      }
    } catch (error) {
      console.error('Error deleting note:', error);
      showToastMessage('Error deleting note');
    } finally {
      showDeleteConfirm = false;
      noteToDelete = null;
    }
  }

  function selectNote(note: Note) {
    selectedNoteId = note.id;
    title = note.title;
    content = note.content;
    tags = note.tags || '';
  }

  function clearSelection() {
    selectedNoteId = null;
    title = '';
    content = '';
    tags = '';
  }

  async function handleSave() {
    if (title === '') {
      showToastMessage('Title is required');
      return;
    }

    try {
      await invoke('save_note', {
        note: {
          id: selectedNoteId,
          title,
          content,
          tags: tags.trim() ? tags : null
        }
      });
      showToastMessage(selectedNoteId ? 'Note updated successfully!' : 'Note created successfully!');
      clearSelection();
      if (isSearching && searchQuery.trim() !== '') {
        await handleSearch();
      } else {
        currentPage = 1;
        hasMoreNotes = true;
        await loadNotes();
      }
    } catch (error) {
      console.error('Error saving note:', error);
      showToastMessage('Error saving note');
    }
  }

  function handleInputAreaKeydown(event: KeyboardEvent) {
    if (event.ctrlKey && event.key.toLowerCase() === 's') {
      event.preventDefault();
      handleSave();
    }
  }

  function handleKeydown(event: KeyboardEvent) {
    // Alt + Q to focus search
    if (event.altKey && event.key.toLowerCase() === 'q') {
      event.preventDefault();
      searchInput?.focus();
    }
  }

  onMount(() => {
    loadNotes();
    // Add global keyboard event listener
    window.addEventListener('keydown', handleKeydown);
    return () => {
      window.removeEventListener('keydown', handleKeydown);
    };
  });
</script>

<main class="m-0 p-0 h-screen w-screen fixed flex flex-col overflow-hidden bg-black">
  <header class="h-[7%] w-full flex justify-between items-center bg-gray-900/90 shadow-md font-mono border-b border-gray-800 px-4">
    <h1 class="text-3xl font-bold text-gray-100">Notes</h1>
    <div class="w-1/3 min-w-[200px]">
      <input
        type="text"
        bind:this={searchInput}
        class="w-full p-2 border border-gray-700 rounded-lg text-base outline-none transition-colors duration-200 focus:border-blue-500 bg-gray-800 text-gray-100 placeholder-gray-400"
        placeholder="Search notes... (Alt+Q)"
        bind:value={searchQuery}
        on:input={handleSearch}
      />
    </div>
  </header>
  
  <div class="h-[93%] w-full flex flex-col bg-gray-900">
    <div 
      class="flex-1 p-4 overflow-y-scroll min-h-0"
      bind:this={notesContainer}
      on:scroll={handleScroll}
    >
      <div class="min-h-[150%] max-w-7xl mx-auto">
        {#if notes.length === 0 && !isLoading}
          <div class="flex justify-center items-start py-8">
            <div class="text-gray-400">
              {searchQuery.trim() ? 'No matching notes found' : 'No notes yet'}
            </div>
          </div>
        {:else}
          <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 auto-rows-min pb-8">
            {#each notes as note}
              <div 
                role="button"
                tabindex="0"
                class="text-left bg-gray-800 rounded-lg p-4 shadow-md border border-gray-700 cursor-pointer hover:bg-gray-700 transition-colors duration-200 {selectedNoteId === note.id ? 'border-blue-500' : ''}"
                on:click={() => selectNote(note)}
                on:keydown={(e) => e.key === 'Enter' && selectNote(note)}
              >
                <div class="flex justify-between items-start mb-2">
                  <h3 class="text-xl font-bold text-gray-100">{note.title}</h3>
                  <button
                    type="button"
                    class="p-1 text-gray-400 hover:text-red-500 transition-colors duration-200"
                    on:click={(e) => {
                      e.stopPropagation();
                      confirmDelete(note, e);
                    }}
                    on:keydown={(e) => {
                      if (e.key === 'Enter') {
                        e.stopPropagation();
                        confirmDelete(note, e);
                      }
                    }}
                    aria-label="Delete note"
                  >
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
                      <path fill-rule="evenodd" d="M9 2a1 1 0 00-.894.553L7.382 4H4a1 1 0 000 2v10a2 2 0 002 2h8a2 2 0 002-2V6a1 1 0 100-2h-3.382l-.724-1.447A1 1 0 0011 2H9zM7 8a1 1 0 012 0v6a1 1 0 11-2 0V8zm5-1a1 1 0 00-1 1v6a1 1 0 102 0V8a1 1 0 00-1-1z" clip-rule="evenodd" />
                    </svg>
                  </button>
                </div>
                <p class="text-gray-300 mb-2 whitespace-pre-wrap line-clamp-3">{note.content}</p>
                {#if note.tags}
                  <div class="flex flex-wrap gap-2 mt-2">
                    {#each note.tags.split(',').map((tag: string) => tag.trim()) as tag}
                      <span class="bg-gray-700 text-gray-300 px-2 py-1 rounded text-sm">{tag}</span>
                    {/each}
                  </div>
                {/if}
                <div class="text-gray-400 text-sm mt-2">{note.created_at}</div>
              </div>
            {/each}
          </div>
          {#if isLoading}
            <div class="flex justify-center items-center py-4">
              <div class="text-gray-400">Loading more notes...</div>
            </div>
          {/if}
        {/if}
      </div>
    </div>

    <div class="w-full bg-gray-900/90 shadow-md border-t border-gray-800 p-4">
      <div class="w-full max-w-4xl mx-auto flex flex-col gap-2">
        <div class="w-full flex justify-between items-center">
          <h2 class="text-xl font-bold text-gray-100">
            {selectedNoteId ? 'Edit Note' : 'New Note'}
          </h2>
          {#if selectedNoteId}
            <button 
              class="px-3 py-1 bg-gray-700 text-gray-300 rounded-lg hover:bg-gray-600 transition-colors duration-200"
              on:click={clearSelection}
            >
              New Note
            </button>
          {/if}
        </div>
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
          rows="3"
          on:keydown={handleInputAreaKeydown}
        ></textarea>
        <textarea 
          class="w-full min-h-[20px] max-h-[10vh] p-2 border border-gray-700 rounded-lg text-base resize-vertical outline-none transition-colors duration-200 focus:border-blue-500 bg-gray-800 text-gray-100 placeholder-gray-400 font-mono"
          bind:value={tags}
          placeholder="Tags (comma separated)..."
          rows="1"
          on:keydown={handleInputAreaKeydown}
        ></textarea>
        <button 
          class="w-full px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors duration-200"
          on:click={handleSave}
        >
          {selectedNoteId ? 'Update Note' : 'Save Note'}
        </button>
      </div>
    </div>
  </div>
</main>

{#if showToast}
  <div class="fixed top-4 right-4 bg-gray-800 text-gray-100 px-5 py-3 rounded-lg shadow-lg z-50 border border-gray-700">
    {toastMessage}
  </div>
{/if}

{#if showDeleteConfirm}
  <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
    <div class="bg-gray-800 rounded-lg p-6 max-w-md w-full mx-4 border border-gray-700">
      <h3 class="text-xl font-bold text-gray-100 mb-4">Delete Note</h3>
      <p class="text-gray-300 mb-6">Are you sure you want to delete "{noteToDelete?.title}"? This action cannot be undone.</p>
      <div class="flex justify-end gap-4">
        <button
          class="px-4 py-2 bg-gray-700 text-gray-300 rounded-lg hover:bg-gray-600 transition-colors duration-200"
          on:click={() => { showDeleteConfirm = false; noteToDelete = null; }}
        >
          Cancel
        </button>
        <button
          class="px-4 py-2 bg-red-600 text-white rounded-lg hover:bg-red-700 transition-colors duration-200"
          on:click={handleDelete}
        >
          Delete
        </button>
      </div>
    </div>
  </div>
{/if}

<style>
  :global(*) {
    scrollbar-width: thin;
    scrollbar-color: rgba(75, 85, 99, 0.5) transparent;
  }

  :global(*::-webkit-scrollbar) {
    width: 8px;
    height: 8px;
  }

  :global(*::-webkit-scrollbar-track) {
    background: transparent;
  }

  :global(*::-webkit-scrollbar-thumb) {
    background-color: rgba(75, 85, 99, 0.5);
    border-radius: 4px;
  }

  :global(*::-webkit-scrollbar-thumb:hover) {
    background-color: rgba(75, 85, 99, 0.7);
  }

  :global(.overflow-y-scroll) {
    scrollbar-gutter: stable;
  }
</style>
