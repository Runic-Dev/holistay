import { writable } from 'svelte/store';
import type { Property } from './models/Property';

export const userStore = writable({
  user: null
});

export const propertyStore = writable<PropertyStore>({
  properties: []
});

type PropertyStore = {
  properties: Property[]
}
