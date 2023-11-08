import { LocalStorageDatabase, RoomGroupRepo, type Database, type Repository } from "./lib/database";
import type RoomGroup from "./models/RoomGroup";

const database: Database = new LocalStorageDatabase(localStorage);

export const roomGroupRepository: Repository<RoomGroup> = new RoomGroupRepo(database);

