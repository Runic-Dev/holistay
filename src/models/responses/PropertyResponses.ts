import type {Property} from "@/models/Property";

export class PropertyResponse {
    success: boolean;
    data: Property;
}