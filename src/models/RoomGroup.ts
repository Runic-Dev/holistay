import type { IsSaveable } from "src/lib/database"
import type { Room } from "src/types"

export default class RoomGroup implements IsSaveable {
  public id: string
  public name: string
  public description: string
  public imageUrl: string
  public rooms: Room[]

  constructor(name: string) {
    this.id = "";
    this.name = name;
  }

  static BlankRoomGroup(): RoomGroup {
    const roomGroup = new RoomGroup("Add RoomGroup");
    roomGroup.id = "blank-roomgroup";
    roomGroup.description = "Add RoomGroup";
    return roomGroup;
  }
}
