import type {Property} from "@/models/Property";

export type Address = {
  street: string,
  city: string,
  state: string,
  postalCode: string,
  country: string
}

export type Contact = {
  name: string,
  phone: string,
  email: string
}

export enum Bed {
  Single,
  Double,
  Queen,
  King
}

export type TileConfig = {
  type: TileType,
  title: string | null,
  image: string | null,
  clickAction: null | (() => void | Promise<void>)
}

export type PropertyResponse = {
  success: boolean,
  data: Property
}

export type RoomGroupResponse = {
  id: string,
  name: string,
  image: string,
  description: string,
  rooms: Room[]
};

export type RoomResponse = {
  id: string,
  room_group_id: string,
  name: string,
  image: string,
  description: string
};

export type ConfirmedPropertyToSend = {
  name: string;
  image: string;
}

