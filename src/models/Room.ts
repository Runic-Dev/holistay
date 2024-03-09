import type { RoomResponse } from "@/types"

export default class Room {
  id: string
  room_group_id: string
  name: string
  description: string | null
  image: string | null

  constructor(id: string, room_group_id: string, name: string, description: string = "", image: string = "") {
    this.id = id;
    this.room_group_id = room_group_id;
    this.name = name;
    this.description = description;
    this.image = image;
  }

  static FromResponse({ id, room_group_id, name, description, image}: RoomResponse): Room {
    return new Room(id, room_group_id, name, description, image);
  }
}
