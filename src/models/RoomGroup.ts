import type { RoomGroupResponse } from "src/types";
import { v4 as uuid } from "uuid";
import type Room from "./Room";

export default class RoomGroup {
  public id: string
  public name: string
  public description: string
  public imageUrl: string
  public rooms: Room[]

  constructor(name: string) {
    this.id = uuid();
    this.name = name;
  }

  static FromRoomGroupResponse(response: RoomGroupResponse) {
    let roomGroup = new RoomGroup(response.name);
    roomGroup.id = response.id;
    roomGroup.imageUrl = response.image;
    roomGroup.description = response.description;
    //roomGroup.rooms = response.
    return roomGroup;
  }
}
