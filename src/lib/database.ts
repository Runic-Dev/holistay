import type RoomGroup from "src/models/RoomGroup";

export interface Database {
  save<T extends IsSaveable>(item: T): T;
  get<T extends IsSaveable>(key: string): T;
}

export interface Repository<T> {
  save(item: T): T;
  getById(id: string): T;
}

export interface IsSaveable {
  id: string;
}

export class LocalStorageDatabase implements Database {
  private localStorage: Storage;

  constructor(localStorage: Storage) {
    this.localStorage = localStorage;
  }

  get<T extends IsSaveable>(key: string): T {
    return JSON.parse(this.localStorage.getItem(key));
  }

  save<T extends IsSaveable>(item: T): T {
    this.localStorage.setItem(item.id, JSON.stringify(item));
    return item;
  }
}

export class RoomGroupRepo implements Repository<RoomGroup> {

  private database: Database;

  constructor(database: Database) {
    this.database = database;
  }

  save(item: RoomGroup): RoomGroup {
    this.database.save(item);
    return item;
  }

  getById(id: string): RoomGroup {
    return this.database.get(id);
  }
}
