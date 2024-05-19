import { writable } from 'svelte/store';
import type { Property } from './models/Property';
import type User from './models/User';

export const userStore = writable<UserStore>({
  user: null
});

export const propertyStore = writable<PropertyStore>({
  properties: []
});

export const displayTopBarStore = writable<boolean>(false);

type UserStore = {
  user: User | null
}

type PropertyStore = {
  properties: Property[]
}
