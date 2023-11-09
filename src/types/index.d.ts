import type { IsSaveable } from "src/lib/database"

export type Property = {
  id: string,
  name: string,
  imageUrl: string,
  address: Address,
  contact: Contact,
  roomGroups: RoomGroup[]
}

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


export type Room = {
  id: string,
  number: number,
  imageUrl: string,
  name: string,
  beds: Bed[]
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
  imageUrl: string | null,
  clickAction: null | (() => void | Promise<void>)
}

