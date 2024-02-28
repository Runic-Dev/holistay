import type RoomGroup from "./RoomGroup"

export class Property  {
  id: string = "";
  name: string = "";
  image: string;
  roomGroups: RoomGroup[]

  constructor(id: string, name: string, image: string = "", roomGroups: RoomGroup[] = []) {
    this.id = id;
    this.name = name;
    this.image = image;
    this.roomGroups = roomGroups;
  }
}
