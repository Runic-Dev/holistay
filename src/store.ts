import { writable } from 'svelte/store';

export const userStore = writable({
  user: null
});

export const propertyStore = writable({
  properties: null
});


