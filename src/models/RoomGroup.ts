import type { IsSaveable } from "src/lib/database";
import type { Room } from "src/types";
import { v4 as uuid } from "uuid";

export default class RoomGroup implements IsSaveable {
  public id: string
  public name: string
  public description: string
  public imageUrl: string
  public rooms: Room[]

  constructor(name: string) {
    this.id = uuid();
    this.name = name;
  }
}
